//! Types for the `CloudFront` service.

/// The [`AWS::CloudFront::CachePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cachepolicy.html) resource type.
#[derive(Debug, Default)]
pub struct CachePolicy {
    properties: CachePolicyProperties
}

/// Properties for the `CachePolicy` resource.
#[derive(Debug, Default)]
pub struct CachePolicyProperties {
    /// Property [`CachePolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cachepolicy.html#cfn-cloudfront-cachepolicy-cachepolicyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_policy_config: ::Value<self::cache_policy::CachePolicyConfig>,
}

impl ::serde::Serialize for CachePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachePolicyConfig", &self.cache_policy_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CachePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CachePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CachePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CachePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_policy_config: Option<::Value<self::cache_policy::CachePolicyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CachePolicyConfig" => {
                            cache_policy_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CachePolicyProperties {
                    cache_policy_config: cache_policy_config.ok_or(::serde::de::Error::missing_field("CachePolicyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CachePolicy {
    type Properties = CachePolicyProperties;
    const TYPE: &'static str = "AWS::CloudFront::CachePolicy";
    fn properties(&self) -> &CachePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CachePolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CachePolicy {}

impl From<CachePolicyProperties> for CachePolicy {
    fn from(properties: CachePolicyProperties) -> CachePolicy {
        CachePolicy { properties }
    }
}

/// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html) resource type.
#[derive(Debug, Default)]
pub struct CloudFrontOriginAccessIdentity {
    properties: CloudFrontOriginAccessIdentityProperties
}

/// Properties for the `CloudFrontOriginAccessIdentity` resource.
#[derive(Debug, Default)]
pub struct CloudFrontOriginAccessIdentityProperties {
    /// Property [`CloudFrontOriginAccessIdentityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html#cfn-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
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
#[derive(Debug, Default)]
pub struct Distribution {
    properties: DistributionProperties
}

/// Properties for the `Distribution` resource.
#[derive(Debug, Default)]
pub struct DistributionProperties {
    /// Property [`DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html#cfn-cloudfront-distribution-distributionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub distribution_config: ::Value<self::distribution::DistributionConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html#cfn-cloudfront-distribution-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
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

/// The [`AWS::CloudFront::Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html) resource type.
#[derive(Debug, Default)]
pub struct Function {
    properties: FunctionProperties
}

/// Properties for the `Function` resource.
#[derive(Debug, Default)]
pub struct FunctionProperties {
    /// Property [`AutoPublish`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-autopublish).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_publish: Option<::Value<bool>>,
    /// Property [`FunctionCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-functioncode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_code: Option<::Value<String>>,
    /// Property [`FunctionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-functionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_config: Option<::Value<self::function::FunctionConfig>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html#cfn-cloudfront-function-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for FunctionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_publish) = self.auto_publish {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoPublish", auto_publish)?;
        }
        if let Some(ref function_code) = self.function_code {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionCode", function_code)?;
        }
        if let Some(ref function_config) = self.function_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionConfig", function_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FunctionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FunctionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FunctionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_publish: Option<::Value<bool>> = None;
                let mut function_code: Option<::Value<String>> = None;
                let mut function_config: Option<::Value<self::function::FunctionConfig>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoPublish" => {
                            auto_publish = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionCode" => {
                            function_code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionConfig" => {
                            function_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FunctionProperties {
                    auto_publish: auto_publish,
                    function_code: function_code,
                    function_config: function_config,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Function {
    type Properties = FunctionProperties;
    const TYPE: &'static str = "AWS::CloudFront::Function";
    fn properties(&self) -> &FunctionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Function {}

impl From<FunctionProperties> for Function {
    fn from(properties: FunctionProperties) -> Function {
        Function { properties }
    }
}

/// The [`AWS::CloudFront::KeyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-keygroup.html) resource type.
#[derive(Debug, Default)]
pub struct KeyGroup {
    properties: KeyGroupProperties
}

/// Properties for the `KeyGroup` resource.
#[derive(Debug, Default)]
pub struct KeyGroupProperties {
    /// Property [`KeyGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-keygroup.html#cfn-cloudfront-keygroup-keygroupconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub key_group_config: ::Value<self::key_group::KeyGroupConfig>,
}

impl ::serde::Serialize for KeyGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyGroupConfig", &self.key_group_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for KeyGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = KeyGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type KeyGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut key_group_config: Option<::Value<self::key_group::KeyGroupConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KeyGroupConfig" => {
                            key_group_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(KeyGroupProperties {
                    key_group_config: key_group_config.ok_or(::serde::de::Error::missing_field("KeyGroupConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for KeyGroup {
    type Properties = KeyGroupProperties;
    const TYPE: &'static str = "AWS::CloudFront::KeyGroup";
    fn properties(&self) -> &KeyGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut KeyGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for KeyGroup {}

impl From<KeyGroupProperties> for KeyGroup {
    fn from(properties: KeyGroupProperties) -> KeyGroup {
        KeyGroup { properties }
    }
}

/// The [`AWS::CloudFront::OriginRequestPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-originrequestpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct OriginRequestPolicy {
    properties: OriginRequestPolicyProperties
}

/// Properties for the `OriginRequestPolicy` resource.
#[derive(Debug, Default)]
pub struct OriginRequestPolicyProperties {
    /// Property [`OriginRequestPolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-originrequestpolicy.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub origin_request_policy_config: ::Value<self::origin_request_policy::OriginRequestPolicyConfig>,
}

impl ::serde::Serialize for OriginRequestPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginRequestPolicyConfig", &self.origin_request_policy_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OriginRequestPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginRequestPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OriginRequestPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OriginRequestPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut origin_request_policy_config: Option<::Value<self::origin_request_policy::OriginRequestPolicyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "OriginRequestPolicyConfig" => {
                            origin_request_policy_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OriginRequestPolicyProperties {
                    origin_request_policy_config: origin_request_policy_config.ok_or(::serde::de::Error::missing_field("OriginRequestPolicyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OriginRequestPolicy {
    type Properties = OriginRequestPolicyProperties;
    const TYPE: &'static str = "AWS::CloudFront::OriginRequestPolicy";
    fn properties(&self) -> &OriginRequestPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OriginRequestPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OriginRequestPolicy {}

impl From<OriginRequestPolicyProperties> for OriginRequestPolicy {
    fn from(properties: OriginRequestPolicyProperties) -> OriginRequestPolicy {
        OriginRequestPolicy { properties }
    }
}

/// The [`AWS::CloudFront::PublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-publickey.html) resource type.
#[derive(Debug, Default)]
pub struct PublicKey {
    properties: PublicKeyProperties
}

/// Properties for the `PublicKey` resource.
#[derive(Debug, Default)]
pub struct PublicKeyProperties {
    /// Property [`PublicKeyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-publickey.html#cfn-cloudfront-publickey-publickeyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_key_config: ::Value<self::public_key::PublicKeyConfig>,
}

impl ::serde::Serialize for PublicKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicKeyConfig", &self.public_key_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PublicKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PublicKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PublicKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut public_key_config: Option<::Value<self::public_key::PublicKeyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PublicKeyConfig" => {
                            public_key_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PublicKeyProperties {
                    public_key_config: public_key_config.ok_or(::serde::de::Error::missing_field("PublicKeyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PublicKey {
    type Properties = PublicKeyProperties;
    const TYPE: &'static str = "AWS::CloudFront::PublicKey";
    fn properties(&self) -> &PublicKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PublicKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PublicKey {}

impl From<PublicKeyProperties> for PublicKey {
    fn from(properties: PublicKeyProperties) -> PublicKey {
        PublicKey { properties }
    }
}

/// The [`AWS::CloudFront::RealtimeLogConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html) resource type.
#[derive(Debug, Default)]
pub struct RealtimeLogConfig {
    properties: RealtimeLogConfigProperties
}

/// Properties for the `RealtimeLogConfig` resource.
#[derive(Debug, Default)]
pub struct RealtimeLogConfigProperties {
    /// Property [`EndPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-endpoints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub end_points: ::ValueList<self::realtime_log_config::EndPoint>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: ::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SamplingRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html#cfn-cloudfront-realtimelogconfig-samplingrate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sampling_rate: ::Value<f64>,
}

impl ::serde::Serialize for RealtimeLogConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndPoints", &self.end_points)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", &self.fields)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRate", &self.sampling_rate)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RealtimeLogConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RealtimeLogConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RealtimeLogConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RealtimeLogConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut end_points: Option<::ValueList<self::realtime_log_config::EndPoint>> = None;
                let mut fields: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut sampling_rate: Option<::Value<f64>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndPoints" => {
                            end_points = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplingRate" => {
                            sampling_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RealtimeLogConfigProperties {
                    end_points: end_points.ok_or(::serde::de::Error::missing_field("EndPoints"))?,
                    fields: fields.ok_or(::serde::de::Error::missing_field("Fields"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    sampling_rate: sampling_rate.ok_or(::serde::de::Error::missing_field("SamplingRate"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RealtimeLogConfig {
    type Properties = RealtimeLogConfigProperties;
    const TYPE: &'static str = "AWS::CloudFront::RealtimeLogConfig";
    fn properties(&self) -> &RealtimeLogConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RealtimeLogConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RealtimeLogConfig {}

impl From<RealtimeLogConfigProperties> for RealtimeLogConfig {
    fn from(properties: RealtimeLogConfigProperties) -> RealtimeLogConfig {
        RealtimeLogConfig { properties }
    }
}

/// The [`AWS::CloudFront::ResponseHeadersPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-responseheaderspolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ResponseHeadersPolicy {
    properties: ResponseHeadersPolicyProperties
}

/// Properties for the `ResponseHeadersPolicy` resource.
#[derive(Debug, Default)]
pub struct ResponseHeadersPolicyProperties {
    /// Property [`ResponseHeadersPolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-responseheaderspolicy.html#cfn-cloudfront-responseheaderspolicy-responseheaderspolicyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_headers_policy_config: ::Value<self::response_headers_policy::ResponseHeadersPolicyConfig>,
}

impl ::serde::Serialize for ResponseHeadersPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseHeadersPolicyConfig", &self.response_headers_policy_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResponseHeadersPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseHeadersPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResponseHeadersPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResponseHeadersPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut response_headers_policy_config: Option<::Value<self::response_headers_policy::ResponseHeadersPolicyConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResponseHeadersPolicyConfig" => {
                            response_headers_policy_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResponseHeadersPolicyProperties {
                    response_headers_policy_config: response_headers_policy_config.ok_or(::serde::de::Error::missing_field("ResponseHeadersPolicyConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResponseHeadersPolicy {
    type Properties = ResponseHeadersPolicyProperties;
    const TYPE: &'static str = "AWS::CloudFront::ResponseHeadersPolicy";
    fn properties(&self) -> &ResponseHeadersPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResponseHeadersPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResponseHeadersPolicy {}

impl From<ResponseHeadersPolicyProperties> for ResponseHeadersPolicy {
    fn from(properties: ResponseHeadersPolicyProperties) -> ResponseHeadersPolicy {
        ResponseHeadersPolicy { properties }
    }
}

/// The [`AWS::CloudFront::StreamingDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html) resource type.
#[derive(Debug, Default)]
pub struct StreamingDistribution {
    properties: StreamingDistributionProperties
}

/// Properties for the `StreamingDistribution` resource.
#[derive(Debug, Default)]
pub struct StreamingDistributionProperties {
    /// Property [`StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub streaming_distribution_config: ::Value<self::streaming_distribution::StreamingDistributionConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html#cfn-cloudfront-streamingdistribution-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
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

pub mod cache_policy {
    //! Property types for the `CachePolicy` resource.

    /// The [`AWS::CloudFront::CachePolicy.CachePolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CachePolicyConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: ::Value<f64>,
        /// Property [`MaxTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-maxttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ttl: ::Value<f64>,
        /// Property [`MinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-minttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_ttl: ::Value<f64>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ParametersInCacheKeyAndForwardedToOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html#cfn-cloudfront-cachepolicy-cachepolicyconfig-parametersincachekeyandforwardedtoorigin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters_in_cache_key_and_forwarded_to_origin: ::Value<ParametersInCacheKeyAndForwardedToOrigin>,
    }

    impl ::codec::SerializeValue for CachePolicyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", &self.default_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", &self.max_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", &self.min_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParametersInCacheKeyAndForwardedToOrigin", &self.parameters_in_cache_key_and_forwarded_to_origin)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CachePolicyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CachePolicyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CachePolicyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CachePolicyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut default_ttl: Option<::Value<f64>> = None;
                    let mut max_ttl: Option<::Value<f64>> = None;
                    let mut min_ttl: Option<::Value<f64>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut parameters_in_cache_key_and_forwarded_to_origin: Option<::Value<ParametersInCacheKeyAndForwardedToOrigin>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTTL" => {
                                max_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTTL" => {
                                min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParametersInCacheKeyAndForwardedToOrigin" => {
                                parameters_in_cache_key_and_forwarded_to_origin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CachePolicyConfig {
                        comment: comment,
                        default_ttl: default_ttl.ok_or(::serde::de::Error::missing_field("DefaultTTL"))?,
                        max_ttl: max_ttl.ok_or(::serde::de::Error::missing_field("MaxTTL"))?,
                        min_ttl: min_ttl.ok_or(::serde::de::Error::missing_field("MinTTL"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        parameters_in_cache_key_and_forwarded_to_origin: parameters_in_cache_key_and_forwarded_to_origin.ok_or(::serde::de::Error::missing_field("ParametersInCacheKeyAndForwardedToOrigin"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CookiesConfig {
        /// Property [`CookieBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html#cfn-cloudfront-cachepolicy-cookiesconfig-cookiebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie_behavior: ::Value<String>,
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html#cfn-cloudfront-cachepolicy-cookiesconfig-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CookiesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieBehavior", &self.cookie_behavior)?;
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookiesConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookiesConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookiesConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookiesConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_behavior: Option<::Value<String>> = None;
                    let mut cookies: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieBehavior" => {
                                cookie_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookiesConfig {
                        cookie_behavior: cookie_behavior.ok_or(::serde::de::Error::missing_field("CookieBehavior"))?,
                        cookies: cookies,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HeadersConfig {
        /// Property [`HeaderBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html#cfn-cloudfront-cachepolicy-headersconfig-headerbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_behavior: ::Value<String>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html#cfn-cloudfront-cachepolicy-headersconfig-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HeadersConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderBehavior", &self.header_behavior)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeadersConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeadersConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeadersConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeadersConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_behavior: Option<::Value<String>> = None;
                    let mut headers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderBehavior" => {
                                header_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeadersConfig {
                        header_behavior: header_behavior.ok_or(::serde::de::Error::missing_field("HeaderBehavior"))?,
                        headers: headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.ParametersInCacheKeyAndForwardedToOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html) property type.
    #[derive(Debug, Default)]
    pub struct ParametersInCacheKeyAndForwardedToOrigin {
        /// Property [`CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-cookiesconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies_config: ::Value<CookiesConfig>,
        /// Property [`EnableAcceptEncodingBrotli`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-enableacceptencodingbrotli).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_accept_encoding_brotli: Option<::Value<bool>>,
        /// Property [`EnableAcceptEncodingGzip`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-enableacceptencodinggzip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_accept_encoding_gzip: ::Value<bool>,
        /// Property [`HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-headersconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers_config: ::Value<HeadersConfig>,
        /// Property [`QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html#cfn-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin-querystringsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings_config: ::Value<QueryStringsConfig>,
    }

    impl ::codec::SerializeValue for ParametersInCacheKeyAndForwardedToOrigin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookiesConfig", &self.cookies_config)?;
            if let Some(ref enable_accept_encoding_brotli) = self.enable_accept_encoding_brotli {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAcceptEncodingBrotli", enable_accept_encoding_brotli)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAcceptEncodingGzip", &self.enable_accept_encoding_gzip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeadersConfig", &self.headers_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringsConfig", &self.query_strings_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParametersInCacheKeyAndForwardedToOrigin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParametersInCacheKeyAndForwardedToOrigin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParametersInCacheKeyAndForwardedToOrigin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParametersInCacheKeyAndForwardedToOrigin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookies_config: Option<::Value<CookiesConfig>> = None;
                    let mut enable_accept_encoding_brotli: Option<::Value<bool>> = None;
                    let mut enable_accept_encoding_gzip: Option<::Value<bool>> = None;
                    let mut headers_config: Option<::Value<HeadersConfig>> = None;
                    let mut query_strings_config: Option<::Value<QueryStringsConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookiesConfig" => {
                                cookies_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableAcceptEncodingBrotli" => {
                                enable_accept_encoding_brotli = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableAcceptEncodingGzip" => {
                                enable_accept_encoding_gzip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeadersConfig" => {
                                headers_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringsConfig" => {
                                query_strings_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParametersInCacheKeyAndForwardedToOrigin {
                        cookies_config: cookies_config.ok_or(::serde::de::Error::missing_field("CookiesConfig"))?,
                        enable_accept_encoding_brotli: enable_accept_encoding_brotli,
                        enable_accept_encoding_gzip: enable_accept_encoding_gzip.ok_or(::serde::de::Error::missing_field("EnableAcceptEncodingGzip"))?,
                        headers_config: headers_config.ok_or(::serde::de::Error::missing_field("HeadersConfig"))?,
                        query_strings_config: query_strings_config.ok_or(::serde::de::Error::missing_field("QueryStringsConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::CachePolicy.QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringsConfig {
        /// Property [`QueryStringBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html#cfn-cloudfront-cachepolicy-querystringsconfig-querystringbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_behavior: ::Value<String>,
        /// Property [`QueryStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html#cfn-cloudfront-cachepolicy-querystringsconfig-querystrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for QueryStringsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringBehavior", &self.query_string_behavior)?;
            if let Some(ref query_strings) = self.query_strings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStrings", query_strings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryStringsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_string_behavior: Option<::Value<String>> = None;
                    let mut query_strings: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryStringBehavior" => {
                                query_string_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStrings" => {
                                query_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryStringsConfig {
                        query_string_behavior: query_string_behavior.ok_or(::serde::de::Error::missing_field("QueryStringBehavior"))?,
                        query_strings: query_strings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cloud_front_origin_access_identity {
    //! Property types for the `CloudFrontOriginAccessIdentity` resource.

    /// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity.CloudFrontOriginAccessIdentityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudFrontOriginAccessIdentityConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html#cfn-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct CacheBehavior {
        /// Property [`AllowedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-allowedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property [`CachePolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-cachepolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_policy_id: Option<::Value<String>>,
        /// Property [`CachedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-cachedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cached_methods: Option<::ValueList<String>>,
        /// Property [`Compress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-compress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compress: Option<::Value<bool>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: Option<::Value<f64>>,
        /// Property [`FieldLevelEncryptionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-fieldlevelencryptionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_level_encryption_id: Option<::Value<String>>,
        /// Property [`ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-forwardedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_values: Option<::Value<ForwardedValues>>,
        /// Property [`FunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-functionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_associations: Option<::ValueList<FunctionAssociation>>,
        /// Property [`LambdaFunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-lambdafunctionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>>,
        /// Property [`MaxTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-maxttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ttl: Option<::Value<f64>>,
        /// Property [`MinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-minttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_ttl: Option<::Value<f64>>,
        /// Property [`OriginRequestPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-originrequestpolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_request_policy_id: Option<::Value<String>>,
        /// Property [`PathPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-pathpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_pattern: ::Value<String>,
        /// Property [`RealtimeLogConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-realtimelogconfigarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub realtime_log_config_arn: Option<::Value<String>>,
        /// Property [`ResponseHeadersPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-responseheaderspolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_headers_policy_id: Option<::Value<String>>,
        /// Property [`SmoothStreaming`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-smoothstreaming).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smooth_streaming: Option<::Value<bool>>,
        /// Property [`TargetOriginId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-targetoriginid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_origin_id: ::Value<String>,
        /// Property [`TrustedKeyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-trustedkeygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_key_groups: Option<::ValueList<String>>,
        /// Property [`TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-trustedsigners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_signers: Option<::ValueList<String>>,
        /// Property [`ViewerProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html#cfn-cloudfront-distribution-cachebehavior-viewerprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub viewer_protocol_policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for CacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref cache_policy_id) = self.cache_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachePolicyId", cache_policy_id)?;
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
            if let Some(ref field_level_encryption_id) = self.field_level_encryption_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldLevelEncryptionId", field_level_encryption_id)?;
            }
            if let Some(ref forwarded_values) = self.forwarded_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedValues", forwarded_values)?;
            }
            if let Some(ref function_associations) = self.function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionAssociations", function_associations)?;
            }
            if let Some(ref lambda_function_associations) = self.lambda_function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionAssociations", lambda_function_associations)?;
            }
            if let Some(ref max_ttl) = self.max_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", max_ttl)?;
            }
            if let Some(ref min_ttl) = self.min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", min_ttl)?;
            }
            if let Some(ref origin_request_policy_id) = self.origin_request_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginRequestPolicyId", origin_request_policy_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathPattern", &self.path_pattern)?;
            if let Some(ref realtime_log_config_arn) = self.realtime_log_config_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RealtimeLogConfigArn", realtime_log_config_arn)?;
            }
            if let Some(ref response_headers_policy_id) = self.response_headers_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseHeadersPolicyId", response_headers_policy_id)?;
            }
            if let Some(ref smooth_streaming) = self.smooth_streaming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothStreaming", smooth_streaming)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOriginId", &self.target_origin_id)?;
            if let Some(ref trusted_key_groups) = self.trusted_key_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedKeyGroups", trusted_key_groups)?;
            }
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
                    let mut cache_policy_id: Option<::Value<String>> = None;
                    let mut cached_methods: Option<::ValueList<String>> = None;
                    let mut compress: Option<::Value<bool>> = None;
                    let mut default_ttl: Option<::Value<f64>> = None;
                    let mut field_level_encryption_id: Option<::Value<String>> = None;
                    let mut forwarded_values: Option<::Value<ForwardedValues>> = None;
                    let mut function_associations: Option<::ValueList<FunctionAssociation>> = None;
                    let mut lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>> = None;
                    let mut max_ttl: Option<::Value<f64>> = None;
                    let mut min_ttl: Option<::Value<f64>> = None;
                    let mut origin_request_policy_id: Option<::Value<String>> = None;
                    let mut path_pattern: Option<::Value<String>> = None;
                    let mut realtime_log_config_arn: Option<::Value<String>> = None;
                    let mut response_headers_policy_id: Option<::Value<String>> = None;
                    let mut smooth_streaming: Option<::Value<bool>> = None;
                    let mut target_origin_id: Option<::Value<String>> = None;
                    let mut trusted_key_groups: Option<::ValueList<String>> = None;
                    let mut trusted_signers: Option<::ValueList<String>> = None;
                    let mut viewer_protocol_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachePolicyId" => {
                                cache_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "FieldLevelEncryptionId" => {
                                field_level_encryption_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedValues" => {
                                forwarded_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionAssociations" => {
                                function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "OriginRequestPolicyId" => {
                                origin_request_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathPattern" => {
                                path_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RealtimeLogConfigArn" => {
                                realtime_log_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseHeadersPolicyId" => {
                                response_headers_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmoothStreaming" => {
                                smooth_streaming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOriginId" => {
                                target_origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedKeyGroups" => {
                                trusted_key_groups = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        cache_policy_id: cache_policy_id,
                        cached_methods: cached_methods,
                        compress: compress,
                        default_ttl: default_ttl,
                        field_level_encryption_id: field_level_encryption_id,
                        forwarded_values: forwarded_values,
                        function_associations: function_associations,
                        lambda_function_associations: lambda_function_associations,
                        max_ttl: max_ttl,
                        min_ttl: min_ttl,
                        origin_request_policy_id: origin_request_policy_id,
                        path_pattern: path_pattern.ok_or(::serde::de::Error::missing_field("PathPattern"))?,
                        realtime_log_config_arn: realtime_log_config_arn,
                        response_headers_policy_id: response_headers_policy_id,
                        smooth_streaming: smooth_streaming,
                        target_origin_id: target_origin_id.ok_or(::serde::de::Error::missing_field("TargetOriginId"))?,
                        trusted_key_groups: trusted_key_groups,
                        trusted_signers: trusted_signers,
                        viewer_protocol_policy: viewer_protocol_policy.ok_or(::serde::de::Error::missing_field("ViewerProtocolPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html) property type.
    #[derive(Debug, Default)]
    pub struct Cookies {
        /// Property [`Forward`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html#cfn-cloudfront-distribution-cookies-forward).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forward: ::Value<String>,
        /// Property [`WhitelistedNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html#cfn-cloudfront-distribution-cookies-whitelistednames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct CustomErrorResponse {
        /// Property [`ErrorCachingMinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-errorcachingminttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_caching_min_ttl: Option<::Value<f64>>,
        /// Property [`ErrorCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-errorcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_code: ::Value<u32>,
        /// Property [`ResponseCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-responsecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_code: Option<::Value<u32>>,
        /// Property [`ResponsePagePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html#cfn-cloudfront-distribution-customerrorresponse-responsepagepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct CustomOriginConfig {
        /// Property [`HTTPPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-httpport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_port: Option<::Value<u32>>,
        /// Property [`HTTPSPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-httpsport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub https_port: Option<::Value<u32>>,
        /// Property [`OriginKeepaliveTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originkeepalivetimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_keepalive_timeout: Option<::Value<u32>>,
        /// Property [`OriginProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_protocol_policy: ::Value<String>,
        /// Property [`OriginReadTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originreadtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_read_timeout: Option<::Value<u32>>,
        /// Property [`OriginSSLProtocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html#cfn-cloudfront-distribution-customoriginconfig-originsslprotocols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct DefaultCacheBehavior {
        /// Property [`AllowedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-allowedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property [`CachePolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-cachepolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_policy_id: Option<::Value<String>>,
        /// Property [`CachedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-cachedmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cached_methods: Option<::ValueList<String>>,
        /// Property [`Compress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-compress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compress: Option<::Value<bool>>,
        /// Property [`DefaultTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-defaultttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_ttl: Option<::Value<f64>>,
        /// Property [`FieldLevelEncryptionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-fieldlevelencryptionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_level_encryption_id: Option<::Value<String>>,
        /// Property [`ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-forwardedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_values: Option<::Value<ForwardedValues>>,
        /// Property [`FunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-functionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_associations: Option<::ValueList<FunctionAssociation>>,
        /// Property [`LambdaFunctionAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-lambdafunctionassociations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>>,
        /// Property [`MaxTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-maxttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_ttl: Option<::Value<f64>>,
        /// Property [`MinTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-minttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_ttl: Option<::Value<f64>>,
        /// Property [`OriginRequestPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-originrequestpolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_request_policy_id: Option<::Value<String>>,
        /// Property [`RealtimeLogConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-realtimelogconfigarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub realtime_log_config_arn: Option<::Value<String>>,
        /// Property [`ResponseHeadersPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-responseheaderspolicyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_headers_policy_id: Option<::Value<String>>,
        /// Property [`SmoothStreaming`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-smoothstreaming).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smooth_streaming: Option<::Value<bool>>,
        /// Property [`TargetOriginId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-targetoriginid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_origin_id: ::Value<String>,
        /// Property [`TrustedKeyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-trustedkeygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_key_groups: Option<::ValueList<String>>,
        /// Property [`TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-trustedsigners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_signers: Option<::ValueList<String>>,
        /// Property [`ViewerProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html#cfn-cloudfront-distribution-defaultcachebehavior-viewerprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub viewer_protocol_policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for DefaultCacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref cache_policy_id) = self.cache_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachePolicyId", cache_policy_id)?;
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
            if let Some(ref field_level_encryption_id) = self.field_level_encryption_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldLevelEncryptionId", field_level_encryption_id)?;
            }
            if let Some(ref forwarded_values) = self.forwarded_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedValues", forwarded_values)?;
            }
            if let Some(ref function_associations) = self.function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionAssociations", function_associations)?;
            }
            if let Some(ref lambda_function_associations) = self.lambda_function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionAssociations", lambda_function_associations)?;
            }
            if let Some(ref max_ttl) = self.max_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", max_ttl)?;
            }
            if let Some(ref min_ttl) = self.min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", min_ttl)?;
            }
            if let Some(ref origin_request_policy_id) = self.origin_request_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginRequestPolicyId", origin_request_policy_id)?;
            }
            if let Some(ref realtime_log_config_arn) = self.realtime_log_config_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RealtimeLogConfigArn", realtime_log_config_arn)?;
            }
            if let Some(ref response_headers_policy_id) = self.response_headers_policy_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseHeadersPolicyId", response_headers_policy_id)?;
            }
            if let Some(ref smooth_streaming) = self.smooth_streaming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothStreaming", smooth_streaming)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOriginId", &self.target_origin_id)?;
            if let Some(ref trusted_key_groups) = self.trusted_key_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedKeyGroups", trusted_key_groups)?;
            }
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
                    let mut cache_policy_id: Option<::Value<String>> = None;
                    let mut cached_methods: Option<::ValueList<String>> = None;
                    let mut compress: Option<::Value<bool>> = None;
                    let mut default_ttl: Option<::Value<f64>> = None;
                    let mut field_level_encryption_id: Option<::Value<String>> = None;
                    let mut forwarded_values: Option<::Value<ForwardedValues>> = None;
                    let mut function_associations: Option<::ValueList<FunctionAssociation>> = None;
                    let mut lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>> = None;
                    let mut max_ttl: Option<::Value<f64>> = None;
                    let mut min_ttl: Option<::Value<f64>> = None;
                    let mut origin_request_policy_id: Option<::Value<String>> = None;
                    let mut realtime_log_config_arn: Option<::Value<String>> = None;
                    let mut response_headers_policy_id: Option<::Value<String>> = None;
                    let mut smooth_streaming: Option<::Value<bool>> = None;
                    let mut target_origin_id: Option<::Value<String>> = None;
                    let mut trusted_key_groups: Option<::ValueList<String>> = None;
                    let mut trusted_signers: Option<::ValueList<String>> = None;
                    let mut viewer_protocol_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachePolicyId" => {
                                cache_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "FieldLevelEncryptionId" => {
                                field_level_encryption_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedValues" => {
                                forwarded_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionAssociations" => {
                                function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "OriginRequestPolicyId" => {
                                origin_request_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RealtimeLogConfigArn" => {
                                realtime_log_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseHeadersPolicyId" => {
                                response_headers_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmoothStreaming" => {
                                smooth_streaming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOriginId" => {
                                target_origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedKeyGroups" => {
                                trusted_key_groups = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        cache_policy_id: cache_policy_id,
                        cached_methods: cached_methods,
                        compress: compress,
                        default_ttl: default_ttl,
                        field_level_encryption_id: field_level_encryption_id,
                        forwarded_values: forwarded_values,
                        function_associations: function_associations,
                        lambda_function_associations: lambda_function_associations,
                        max_ttl: max_ttl,
                        min_ttl: min_ttl,
                        origin_request_policy_id: origin_request_policy_id,
                        realtime_log_config_arn: realtime_log_config_arn,
                        response_headers_policy_id: response_headers_policy_id,
                        smooth_streaming: smooth_streaming,
                        target_origin_id: target_origin_id.ok_or(::serde::de::Error::missing_field("TargetOriginId"))?,
                        trusted_key_groups: trusted_key_groups,
                        trusted_signers: trusted_signers,
                        viewer_protocol_policy: viewer_protocol_policy.ok_or(::serde::de::Error::missing_field("ViewerProtocolPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DistributionConfig {
        /// Property [`Aliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-aliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aliases: Option<::ValueList<String>>,
        /// Property [`CNAMEs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-cnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cnam_es: Option<::ValueList<String>>,
        /// Property [`CacheBehaviors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-cachebehaviors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_behaviors: Option<::ValueList<CacheBehavior>>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`CustomErrorResponses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-customerrorresponses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_error_responses: Option<::ValueList<CustomErrorResponse>>,
        /// Property [`CustomOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-customorigin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_origin: Option<::Value<LegacyCustomOrigin>>,
        /// Property [`DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-defaultcachebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_cache_behavior: Option<::Value<DefaultCacheBehavior>>,
        /// Property [`DefaultRootObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-defaultrootobject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_root_object: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`HttpVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-httpversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_version: Option<::Value<String>>,
        /// Property [`IPV6Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-ipv6enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6_enabled: Option<::Value<bool>>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<::Value<Logging>>,
        /// Property [`OriginGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-origingroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_groups: Option<::Value<OriginGroups>>,
        /// Property [`Origins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-origins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origins: Option<::ValueList<Origin>>,
        /// Property [`PriceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-priceclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub price_class: Option<::Value<String>>,
        /// Property [`Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-restrictions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restrictions: Option<::Value<Restrictions>>,
        /// Property [`S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-s3origin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_origin: Option<::Value<LegacyS3Origin>>,
        /// Property [`ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-viewercertificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub viewer_certificate: Option<::Value<ViewerCertificate>>,
        /// Property [`WebACLId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html#cfn-cloudfront-distribution-distributionconfig-webaclid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_acl_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DistributionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aliases) = self.aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aliases", aliases)?;
            }
            if let Some(ref cnam_es) = self.cnam_es {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CNAMEs", cnam_es)?;
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
            if let Some(ref custom_origin) = self.custom_origin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomOrigin", custom_origin)?;
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
            if let Some(ref origin_groups) = self.origin_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginGroups", origin_groups)?;
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
            if let Some(ref s3_origin) = self.s3_origin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Origin", s3_origin)?;
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
                    let mut cnam_es: Option<::ValueList<String>> = None;
                    let mut cache_behaviors: Option<::ValueList<CacheBehavior>> = None;
                    let mut comment: Option<::Value<String>> = None;
                    let mut custom_error_responses: Option<::ValueList<CustomErrorResponse>> = None;
                    let mut custom_origin: Option<::Value<LegacyCustomOrigin>> = None;
                    let mut default_cache_behavior: Option<::Value<DefaultCacheBehavior>> = None;
                    let mut default_root_object: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut http_version: Option<::Value<String>> = None;
                    let mut ipv6_enabled: Option<::Value<bool>> = None;
                    let mut logging: Option<::Value<Logging>> = None;
                    let mut origin_groups: Option<::Value<OriginGroups>> = None;
                    let mut origins: Option<::ValueList<Origin>> = None;
                    let mut price_class: Option<::Value<String>> = None;
                    let mut restrictions: Option<::Value<Restrictions>> = None;
                    let mut s3_origin: Option<::Value<LegacyS3Origin>> = None;
                    let mut viewer_certificate: Option<::Value<ViewerCertificate>> = None;
                    let mut web_acl_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Aliases" => {
                                aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CNAMEs" => {
                                cnam_es = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "CustomOrigin" => {
                                custom_origin = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "OriginGroups" => {
                                origin_groups = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "S3Origin" => {
                                s3_origin = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        cnam_es: cnam_es,
                        cache_behaviors: cache_behaviors,
                        comment: comment,
                        custom_error_responses: custom_error_responses,
                        custom_origin: custom_origin,
                        default_cache_behavior: default_cache_behavior,
                        default_root_object: default_root_object,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        http_version: http_version,
                        ipv6_enabled: ipv6_enabled,
                        logging: logging,
                        origin_groups: origin_groups,
                        origins: origins,
                        price_class: price_class,
                        restrictions: restrictions,
                        s3_origin: s3_origin,
                        viewer_certificate: viewer_certificate,
                        web_acl_id: web_acl_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html) property type.
    #[derive(Debug, Default)]
    pub struct ForwardedValues {
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<::Value<Cookies>>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<String>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: ::Value<bool>,
        /// Property [`QueryStringCacheKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html#cfn-cloudfront-distribution-forwardedvalues-querystringcachekeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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

    /// The [`AWS::CloudFront::Distribution.FunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionAssociation {
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html#cfn-cloudfront-distribution-functionassociation-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: Option<::Value<String>>,
        /// Property [`FunctionARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html#cfn-cloudfront-distribution-functionassociation-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FunctionAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref event_type) = self.event_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
            }
            if let Some(ref function_arn) = self.function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionARN", function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FunctionAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_type: Option<::Value<String>> = None;
                    let mut function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionARN" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionAssociation {
                        event_type: event_type,
                        function_arn: function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoRestriction {
        /// Property [`Locations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html#cfn-cloudfront-distribution-georestriction-locations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locations: Option<::ValueList<String>>,
        /// Property [`RestrictionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html#cfn-cloudfront-distribution-georestriction-restrictiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct LambdaFunctionAssociation {
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html#cfn-cloudfront-distribution-lambdafunctionassociation-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: Option<::Value<String>>,
        /// Property [`IncludeBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html#cfn-cloudfront-distribution-lambdafunctionassociation-includebody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_body: Option<::Value<bool>>,
        /// Property [`LambdaFunctionARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html#cfn-cloudfront-distribution-lambdafunctionassociation-lambdafunctionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaFunctionAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref event_type) = self.event_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
            }
            if let Some(ref include_body) = self.include_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeBody", include_body)?;
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
                    let mut include_body: Option<::Value<bool>> = None;
                    let mut lambda_function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeBody" => {
                                include_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionARN" => {
                                lambda_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaFunctionAssociation {
                        event_type: event_type,
                        include_body: include_body,
                        lambda_function_arn: lambda_function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.LegacyCustomOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html) property type.
    #[derive(Debug, Default)]
    pub struct LegacyCustomOrigin {
        /// Property [`DNSName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-dnsname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: ::Value<String>,
        /// Property [`HTTPPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-httpport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_port: Option<::Value<u32>>,
        /// Property [`HTTPSPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-httpsport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub https_port: Option<::Value<u32>>,
        /// Property [`OriginProtocolPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-originprotocolpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_protocol_policy: ::Value<String>,
        /// Property [`OriginSSLProtocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html#cfn-cloudfront-distribution-legacycustomorigin-originsslprotocols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_ssl_protocols: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for LegacyCustomOrigin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            if let Some(ref http_port) = self.http_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPPort", http_port)?;
            }
            if let Some(ref https_port) = self.https_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPSPort", https_port)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginProtocolPolicy", &self.origin_protocol_policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginSSLProtocols", &self.origin_ssl_protocols)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LegacyCustomOrigin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LegacyCustomOrigin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LegacyCustomOrigin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LegacyCustomOrigin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut http_port: Option<::Value<u32>> = None;
                    let mut https_port: Option<::Value<u32>> = None;
                    let mut origin_protocol_policy: Option<::Value<String>> = None;
                    let mut origin_ssl_protocols: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPPort" => {
                                http_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPSPort" => {
                                https_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginProtocolPolicy" => {
                                origin_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginSSLProtocols" => {
                                origin_ssl_protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LegacyCustomOrigin {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        http_port: http_port,
                        https_port: https_port,
                        origin_protocol_policy: origin_protocol_policy.ok_or(::serde::de::Error::missing_field("OriginProtocolPolicy"))?,
                        origin_ssl_protocols: origin_ssl_protocols.ok_or(::serde::de::Error::missing_field("OriginSSLProtocols"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.LegacyS3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html) property type.
    #[derive(Debug, Default)]
    pub struct LegacyS3Origin {
        /// Property [`DNSName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html#cfn-cloudfront-distribution-legacys3origin-dnsname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: ::Value<String>,
        /// Property [`OriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html#cfn-cloudfront-distribution-legacys3origin-originaccessidentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_access_identity: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LegacyS3Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DNSName", &self.dns_name)?;
            if let Some(ref origin_access_identity) = self.origin_access_identity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginAccessIdentity", origin_access_identity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LegacyS3Origin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LegacyS3Origin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LegacyS3Origin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LegacyS3Origin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut origin_access_identity: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DNSName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginAccessIdentity" => {
                                origin_access_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LegacyS3Origin {
                        dns_name: dns_name.ok_or(::serde::de::Error::missing_field("DNSName"))?,
                        origin_access_identity: origin_access_identity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html) property type.
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html#cfn-cloudfront-distribution-logging-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`IncludeCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html#cfn-cloudfront-distribution-logging-includecookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_cookies: Option<::Value<bool>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html#cfn-cloudfront-distribution-logging-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct Origin {
        /// Property [`ConnectionAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-connectionattempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_attempts: Option<::Value<u32>>,
        /// Property [`ConnectionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-connectiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_timeout: Option<::Value<u32>>,
        /// Property [`CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-customoriginconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_origin_config: Option<::Value<CustomOriginConfig>>,
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`OriginCustomHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-origincustomheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_custom_headers: Option<::ValueList<OriginCustomHeader>>,
        /// Property [`OriginPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-originpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_path: Option<::Value<String>>,
        /// Property [`OriginShield`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-originshield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_shield: Option<::Value<OriginShield>>,
        /// Property [`S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html#cfn-cloudfront-distribution-origin-s3originconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_origin_config: Option<::Value<S3OriginConfig>>,
    }

    impl ::codec::SerializeValue for Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_attempts) = self.connection_attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionAttempts", connection_attempts)?;
            }
            if let Some(ref connection_timeout) = self.connection_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionTimeout", connection_timeout)?;
            }
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
            if let Some(ref origin_shield) = self.origin_shield {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginShield", origin_shield)?;
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
                    let mut connection_attempts: Option<::Value<u32>> = None;
                    let mut connection_timeout: Option<::Value<u32>> = None;
                    let mut custom_origin_config: Option<::Value<CustomOriginConfig>> = None;
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut origin_custom_headers: Option<::ValueList<OriginCustomHeader>> = None;
                    let mut origin_path: Option<::Value<String>> = None;
                    let mut origin_shield: Option<::Value<OriginShield>> = None;
                    let mut s3_origin_config: Option<::Value<S3OriginConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionAttempts" => {
                                connection_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionTimeout" => {
                                connection_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
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
                            "OriginShield" => {
                                origin_shield = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OriginConfig" => {
                                s3_origin_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Origin {
                        connection_attempts: connection_attempts,
                        connection_timeout: connection_timeout,
                        custom_origin_config: custom_origin_config,
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        origin_custom_headers: origin_custom_headers,
                        origin_path: origin_path,
                        origin_shield: origin_shield,
                        s3_origin_config: s3_origin_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginCustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginCustomHeader {
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html#cfn-cloudfront-distribution-origincustomheader-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: ::Value<String>,
        /// Property [`HeaderValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html#cfn-cloudfront-distribution-origincustomheader-headervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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

    /// The [`AWS::CloudFront::Distribution.OriginGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroup {
        /// Property [`FailoverCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html#cfn-cloudfront-distribution-origingroup-failovercriteria).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover_criteria: ::Value<OriginGroupFailoverCriteria>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html#cfn-cloudfront-distribution-origingroup-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Members`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html#cfn-cloudfront-distribution-origingroup-members).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub members: ::Value<OriginGroupMembers>,
    }

    impl ::codec::SerializeValue for OriginGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailoverCriteria", &self.failover_criteria)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Members", &self.members)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failover_criteria: Option<::Value<OriginGroupFailoverCriteria>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut members: Option<::Value<OriginGroupMembers>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailoverCriteria" => {
                                failover_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Members" => {
                                members = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroup {
                        failover_criteria: failover_criteria.ok_or(::serde::de::Error::missing_field("FailoverCriteria"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        members: members.ok_or(::serde::de::Error::missing_field("Members"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroupFailoverCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupfailovercriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroupFailoverCriteria {
        /// Property [`StatusCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupfailovercriteria.html#cfn-cloudfront-distribution-origingroupfailovercriteria-statuscodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_codes: ::Value<StatusCodes>,
    }

    impl ::codec::SerializeValue for OriginGroupFailoverCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCodes", &self.status_codes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginGroupFailoverCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroupFailoverCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroupFailoverCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroupFailoverCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status_codes: Option<::Value<StatusCodes>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StatusCodes" => {
                                status_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroupFailoverCriteria {
                        status_codes: status_codes.ok_or(::serde::de::Error::missing_field("StatusCodes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroupMember`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmember.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroupMember {
        /// Property [`OriginId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmember.html#cfn-cloudfront-distribution-origingroupmember-originid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for OriginGroupMember {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginId", &self.origin_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginGroupMember {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroupMember, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroupMember;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroupMember")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut origin_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OriginId" => {
                                origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroupMember {
                        origin_id: origin_id.ok_or(::serde::de::Error::missing_field("OriginId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroupMembers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroupMembers {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html#cfn-cloudfront-distribution-origingroupmembers-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<OriginGroupMember>,
        /// Property [`Quantity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html#cfn-cloudfront-distribution-origingroupmembers-quantity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quantity: ::Value<u32>,
    }

    impl ::codec::SerializeValue for OriginGroupMembers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quantity", &self.quantity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginGroupMembers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroupMembers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroupMembers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroupMembers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<OriginGroupMember>> = None;
                    let mut quantity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Quantity" => {
                                quantity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroupMembers {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                        quantity: quantity.ok_or(::serde::de::Error::missing_field("Quantity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginGroups {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html#cfn-cloudfront-distribution-origingroups-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: Option<::ValueList<OriginGroup>>,
        /// Property [`Quantity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html#cfn-cloudfront-distribution-origingroups-quantity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quantity: ::Value<u32>,
    }

    impl ::codec::SerializeValue for OriginGroups {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref items) = self.items {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", items)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quantity", &self.quantity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginGroups {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginGroups, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginGroups;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginGroups")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<OriginGroup>> = None;
                    let mut quantity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Quantity" => {
                                quantity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginGroups {
                        items: items,
                        quantity: quantity.ok_or(::serde::de::Error::missing_field("Quantity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginShield`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginShield {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html#cfn-cloudfront-distribution-originshield-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`OriginShieldRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html#cfn-cloudfront-distribution-originshield-originshieldregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_shield_region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OriginShield {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref origin_shield_region) = self.origin_shield_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginShieldRegion", origin_shield_region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginShield {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginShield, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginShield;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginShield")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut origin_shield_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginShieldRegion" => {
                                origin_shield_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginShield {
                        enabled: enabled,
                        origin_shield_region: origin_shield_region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html) property type.
    #[derive(Debug, Default)]
    pub struct Restrictions {
        /// Property [`GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html#cfn-cloudfront-distribution-restrictions-georestriction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct S3OriginConfig {
        /// Property [`OriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html#cfn-cloudfront-distribution-s3originconfig-originaccessidentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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

    /// The [`AWS::CloudFront::Distribution.StatusCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html) property type.
    #[derive(Debug, Default)]
    pub struct StatusCodes {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html#cfn-cloudfront-distribution-statuscodes-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<u32>,
        /// Property [`Quantity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html#cfn-cloudfront-distribution-statuscodes-quantity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quantity: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StatusCodes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quantity", &self.quantity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatusCodes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatusCodes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatusCodes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatusCodes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<u32>> = None;
                    let mut quantity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Quantity" => {
                                quantity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatusCodes {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                        quantity: quantity.ok_or(::serde::de::Error::missing_field("Quantity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ViewerCertificate {
        /// Property [`AcmCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-acmcertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm_certificate_arn: Option<::Value<String>>,
        /// Property [`CloudFrontDefaultCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-cloudfrontdefaultcertificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_front_default_certificate: Option<::Value<bool>>,
        /// Property [`IamCertificateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-iamcertificateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_certificate_id: Option<::Value<String>>,
        /// Property [`MinimumProtocolVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-minimumprotocolversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_protocol_version: Option<::Value<String>>,
        /// Property [`SslSupportMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html#cfn-cloudfront-distribution-viewercertificate-sslsupportmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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

pub mod function {
    //! Property types for the `Function` resource.

    /// The [`AWS::CloudFront::Function.FunctionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html#cfn-cloudfront-function-functionconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: ::Value<String>,
        /// Property [`Runtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html#cfn-cloudfront-function-functionconfig-runtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime: ::Value<String>,
    }

    impl ::codec::SerializeValue for FunctionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", &self.runtime)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FunctionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut runtime: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Runtime" => {
                                runtime = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionConfig {
                        comment: comment.ok_or(::serde::de::Error::missing_field("Comment"))?,
                        runtime: runtime.ok_or(::serde::de::Error::missing_field("Runtime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Function.FunctionMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionmetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct FunctionMetadata {
        /// Property [`FunctionARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionmetadata.html#cfn-cloudfront-function-functionmetadata-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FunctionMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref function_arn) = self.function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionARN", function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FunctionMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FunctionMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FunctionMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionARN" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FunctionMetadata {
                        function_arn: function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod key_group {
    //! Property types for the `KeyGroup` resource.

    /// The [`AWS::CloudFront::KeyGroup.KeyGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyGroupConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html#cfn-cloudfront-keygroup-keygroupconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html#cfn-cloudfront-keygroup-keygroupconfig-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html#cfn-cloudfront-keygroup-keygroupconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for KeyGroupConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyGroupConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyGroupConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyGroupConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyGroupConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut items: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyGroupConfig {
                        comment: comment,
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod origin_request_policy {
    //! Property types for the `OriginRequestPolicy` resource.

    /// The [`AWS::CloudFront::OriginRequestPolicy.CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CookiesConfig {
        /// Property [`CookieBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html#cfn-cloudfront-originrequestpolicy-cookiesconfig-cookiebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie_behavior: ::Value<String>,
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html#cfn-cloudfront-originrequestpolicy-cookiesconfig-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CookiesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieBehavior", &self.cookie_behavior)?;
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookiesConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookiesConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookiesConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookiesConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_behavior: Option<::Value<String>> = None;
                    let mut cookies: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieBehavior" => {
                                cookie_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookiesConfig {
                        cookie_behavior: cookie_behavior.ok_or(::serde::de::Error::missing_field("CookieBehavior"))?,
                        cookies: cookies,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::OriginRequestPolicy.HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HeadersConfig {
        /// Property [`HeaderBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html#cfn-cloudfront-originrequestpolicy-headersconfig-headerbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_behavior: ::Value<String>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html#cfn-cloudfront-originrequestpolicy-headersconfig-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HeadersConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderBehavior", &self.header_behavior)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeadersConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeadersConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeadersConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeadersConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_behavior: Option<::Value<String>> = None;
                    let mut headers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderBehavior" => {
                                header_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeadersConfig {
                        header_behavior: header_behavior.ok_or(::serde::de::Error::missing_field("HeaderBehavior"))?,
                        headers: headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::OriginRequestPolicy.OriginRequestPolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginRequestPolicyConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`CookiesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-cookiesconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies_config: ::Value<CookiesConfig>,
        /// Property [`HeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-headersconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers_config: ::Value<HeadersConfig>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html#cfn-cloudfront-originrequestpolicy-originrequestpolicyconfig-querystringsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings_config: ::Value<QueryStringsConfig>,
    }

    impl ::codec::SerializeValue for OriginRequestPolicyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookiesConfig", &self.cookies_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeadersConfig", &self.headers_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringsConfig", &self.query_strings_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginRequestPolicyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginRequestPolicyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginRequestPolicyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginRequestPolicyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut cookies_config: Option<::Value<CookiesConfig>> = None;
                    let mut headers_config: Option<::Value<HeadersConfig>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut query_strings_config: Option<::Value<QueryStringsConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CookiesConfig" => {
                                cookies_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeadersConfig" => {
                                headers_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringsConfig" => {
                                query_strings_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginRequestPolicyConfig {
                        comment: comment,
                        cookies_config: cookies_config.ok_or(::serde::de::Error::missing_field("CookiesConfig"))?,
                        headers_config: headers_config.ok_or(::serde::de::Error::missing_field("HeadersConfig"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        query_strings_config: query_strings_config.ok_or(::serde::de::Error::missing_field("QueryStringsConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::OriginRequestPolicy.QueryStringsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringsConfig {
        /// Property [`QueryStringBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html#cfn-cloudfront-originrequestpolicy-querystringsconfig-querystringbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_behavior: ::Value<String>,
        /// Property [`QueryStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html#cfn-cloudfront-originrequestpolicy-querystringsconfig-querystrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_strings: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for QueryStringsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringBehavior", &self.query_string_behavior)?;
            if let Some(ref query_strings) = self.query_strings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStrings", query_strings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryStringsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_string_behavior: Option<::Value<String>> = None;
                    let mut query_strings: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryStringBehavior" => {
                                query_string_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStrings" => {
                                query_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryStringsConfig {
                        query_string_behavior: query_string_behavior.ok_or(::serde::de::Error::missing_field("QueryStringBehavior"))?,
                        query_strings: query_strings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod public_key {
    //! Property types for the `PublicKey` resource.

    /// The [`AWS::CloudFront::PublicKey.PublicKeyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PublicKeyConfig {
        /// Property [`CallerReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-callerreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caller_reference: ::Value<String>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`EncodedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-encodedkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encoded_key: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html#cfn-cloudfront-publickey-publickeyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PublicKeyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CallerReference", &self.caller_reference)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncodedKey", &self.encoded_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublicKeyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicKeyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublicKeyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublicKeyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut caller_reference: Option<::Value<String>> = None;
                    let mut comment: Option<::Value<String>> = None;
                    let mut encoded_key: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CallerReference" => {
                                caller_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncodedKey" => {
                                encoded_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublicKeyConfig {
                        caller_reference: caller_reference.ok_or(::serde::de::Error::missing_field("CallerReference"))?,
                        comment: comment,
                        encoded_key: encoded_key.ok_or(::serde::de::Error::missing_field("EncodedKey"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod realtime_log_config {
    //! Property types for the `RealtimeLogConfig` resource.

    /// The [`AWS::CloudFront::RealtimeLogConfig.EndPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct EndPoint {
        /// Property [`KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html#cfn-cloudfront-realtimelogconfig-endpoint-kinesisstreamconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_stream_config: ::Value<KinesisStreamConfig>,
        /// Property [`StreamType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html#cfn-cloudfront-realtimelogconfig-endpoint-streamtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for EndPoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamConfig", &self.kinesis_stream_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamType", &self.stream_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndPoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndPoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndPoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndPoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kinesis_stream_config: Option<::Value<KinesisStreamConfig>> = None;
                    let mut stream_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KinesisStreamConfig" => {
                                kinesis_stream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamType" => {
                                stream_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndPoint {
                        kinesis_stream_config: kinesis_stream_config.ok_or(::serde::de::Error::missing_field("KinesisStreamConfig"))?,
                        stream_type: stream_type.ok_or(::serde::de::Error::missing_field("StreamType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::RealtimeLogConfig.KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamConfig {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html#cfn-cloudfront-realtimelogconfig-kinesisstreamconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html#cfn-cloudfront-realtimelogconfig-kinesisstreamconfig-streamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", &self.stream_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamConfig {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        stream_arn: stream_arn.ok_or(::serde::de::Error::missing_field("StreamArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod response_headers_policy {
    //! Property types for the `ResponseHeadersPolicy` resource.

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.AccessControlAllowHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolallowheaders.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlAllowHeaders {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolallowheaders.html#cfn-cloudfront-responseheaderspolicy-accesscontrolallowheaders-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AccessControlAllowHeaders {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlAllowHeaders {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlAllowHeaders, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlAllowHeaders;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlAllowHeaders")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlAllowHeaders {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.AccessControlAllowMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolallowmethods.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlAllowMethods {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolallowmethods.html#cfn-cloudfront-responseheaderspolicy-accesscontrolallowmethods-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AccessControlAllowMethods {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlAllowMethods {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlAllowMethods, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlAllowMethods;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlAllowMethods")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlAllowMethods {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.AccessControlAllowOrigins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolalloworigins.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlAllowOrigins {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolalloworigins.html#cfn-cloudfront-responseheaderspolicy-accesscontrolalloworigins-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AccessControlAllowOrigins {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlAllowOrigins {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlAllowOrigins, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlAllowOrigins;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlAllowOrigins")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlAllowOrigins {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.AccessControlExposeHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolexposeheaders.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlExposeHeaders {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolexposeheaders.html#cfn-cloudfront-responseheaderspolicy-accesscontrolexposeheaders-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AccessControlExposeHeaders {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlExposeHeaders {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlExposeHeaders, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlExposeHeaders;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlExposeHeaders")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlExposeHeaders {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.ContentSecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contentsecuritypolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ContentSecurityPolicy {
        /// Property [`ContentSecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contentsecuritypolicy.html#cfn-cloudfront-responseheaderspolicy-contentsecuritypolicy-contentsecuritypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_security_policy: ::Value<String>,
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contentsecuritypolicy.html#cfn-cloudfront-responseheaderspolicy-contentsecuritypolicy-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ContentSecurityPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentSecurityPolicy", &self.content_security_policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContentSecurityPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContentSecurityPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContentSecurityPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContentSecurityPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_security_policy: Option<::Value<String>> = None;
                    let mut r#override: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentSecurityPolicy" => {
                                content_security_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContentSecurityPolicy {
                        content_security_policy: content_security_policy.ok_or(::serde::de::Error::missing_field("ContentSecurityPolicy"))?,
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.ContentTypeOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contenttypeoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ContentTypeOptions {
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contenttypeoptions.html#cfn-cloudfront-responseheaderspolicy-contenttypeoptions-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ContentTypeOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContentTypeOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContentTypeOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContentTypeOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContentTypeOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#override: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContentTypeOptions {
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.CorsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CorsConfig {
        /// Property [`AccessControlAllowCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-accesscontrolallowcredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_allow_credentials: ::Value<bool>,
        /// Property [`AccessControlAllowHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-accesscontrolallowheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_allow_headers: ::Value<AccessControlAllowHeaders>,
        /// Property [`AccessControlAllowMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-accesscontrolallowmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_allow_methods: ::Value<AccessControlAllowMethods>,
        /// Property [`AccessControlAllowOrigins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-accesscontrolalloworigins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_allow_origins: ::Value<AccessControlAllowOrigins>,
        /// Property [`AccessControlExposeHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-accesscontrolexposeheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_expose_headers: Option<::Value<AccessControlExposeHeaders>>,
        /// Property [`AccessControlMaxAgeSec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-accesscontrolmaxagesec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_max_age_sec: Option<::Value<u32>>,
        /// Property [`OriginOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html#cfn-cloudfront-responseheaderspolicy-corsconfig-originoverride).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_override: ::Value<bool>,
    }

    impl ::codec::SerializeValue for CorsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlAllowCredentials", &self.access_control_allow_credentials)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlAllowHeaders", &self.access_control_allow_headers)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlAllowMethods", &self.access_control_allow_methods)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlAllowOrigins", &self.access_control_allow_origins)?;
            if let Some(ref access_control_expose_headers) = self.access_control_expose_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlExposeHeaders", access_control_expose_headers)?;
            }
            if let Some(ref access_control_max_age_sec) = self.access_control_max_age_sec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlMaxAgeSec", access_control_max_age_sec)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginOverride", &self.origin_override)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CorsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CorsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CorsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CorsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_control_allow_credentials: Option<::Value<bool>> = None;
                    let mut access_control_allow_headers: Option<::Value<AccessControlAllowHeaders>> = None;
                    let mut access_control_allow_methods: Option<::Value<AccessControlAllowMethods>> = None;
                    let mut access_control_allow_origins: Option<::Value<AccessControlAllowOrigins>> = None;
                    let mut access_control_expose_headers: Option<::Value<AccessControlExposeHeaders>> = None;
                    let mut access_control_max_age_sec: Option<::Value<u32>> = None;
                    let mut origin_override: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessControlAllowCredentials" => {
                                access_control_allow_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessControlAllowHeaders" => {
                                access_control_allow_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessControlAllowMethods" => {
                                access_control_allow_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessControlAllowOrigins" => {
                                access_control_allow_origins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessControlExposeHeaders" => {
                                access_control_expose_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessControlMaxAgeSec" => {
                                access_control_max_age_sec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginOverride" => {
                                origin_override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CorsConfig {
                        access_control_allow_credentials: access_control_allow_credentials.ok_or(::serde::de::Error::missing_field("AccessControlAllowCredentials"))?,
                        access_control_allow_headers: access_control_allow_headers.ok_or(::serde::de::Error::missing_field("AccessControlAllowHeaders"))?,
                        access_control_allow_methods: access_control_allow_methods.ok_or(::serde::de::Error::missing_field("AccessControlAllowMethods"))?,
                        access_control_allow_origins: access_control_allow_origins.ok_or(::serde::de::Error::missing_field("AccessControlAllowOrigins"))?,
                        access_control_expose_headers: access_control_expose_headers,
                        access_control_max_age_sec: access_control_max_age_sec,
                        origin_override: origin_override.ok_or(::serde::de::Error::missing_field("OriginOverride"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.CustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheader.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomHeader {
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheader.html#cfn-cloudfront-responseheaderspolicy-customheader-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: ::Value<String>,
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheader.html#cfn-cloudfront-responseheaderspolicy-customheader-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheader.html#cfn-cloudfront-responseheaderspolicy-customheader-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", &self.header)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header: Option<::Value<String>> = None;
                    let mut r#override: Option<::Value<bool>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomHeader {
                        header: header.ok_or(::serde::de::Error::missing_field("Header"))?,
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.CustomHeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheadersconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomHeadersConfig {
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheadersconfig.html#cfn-cloudfront-responseheaderspolicy-customheadersconfig-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: ::ValueList<CustomHeader>,
    }

    impl ::codec::SerializeValue for CustomHeadersConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomHeadersConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomHeadersConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomHeadersConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomHeadersConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut items: Option<::ValueList<CustomHeader>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomHeadersConfig {
                        items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.FrameOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-frameoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameOptions {
        /// Property [`FrameOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-frameoptions.html#cfn-cloudfront-responseheaderspolicy-frameoptions-frameoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_option: ::Value<String>,
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-frameoptions.html#cfn-cloudfront-responseheaderspolicy-frameoptions-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
    }

    impl ::codec::SerializeValue for FrameOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameOption", &self.frame_option)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut frame_option: Option<::Value<String>> = None;
                    let mut r#override: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FrameOption" => {
                                frame_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameOptions {
                        frame_option: frame_option.ok_or(::serde::de::Error::missing_field("FrameOption"))?,
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.ReferrerPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-referrerpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ReferrerPolicy {
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-referrerpolicy.html#cfn-cloudfront-responseheaderspolicy-referrerpolicy-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
        /// Property [`ReferrerPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-referrerpolicy.html#cfn-cloudfront-responseheaderspolicy-referrerpolicy-referrerpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub referrer_policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReferrerPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferrerPolicy", &self.referrer_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReferrerPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferrerPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReferrerPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReferrerPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#override: Option<::Value<bool>> = None;
                    let mut referrer_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReferrerPolicy" => {
                                referrer_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReferrerPolicy {
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                        referrer_policy: referrer_policy.ok_or(::serde::de::Error::missing_field("ReferrerPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.ResponseHeadersPolicyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseHeadersPolicyConfig {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html#cfn-cloudfront-responseheaderspolicy-responseheaderspolicyconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`CorsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html#cfn-cloudfront-responseheaderspolicy-responseheaderspolicyconfig-corsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cors_config: Option<::Value<CorsConfig>>,
        /// Property [`CustomHeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html#cfn-cloudfront-responseheaderspolicy-responseheaderspolicyconfig-customheadersconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_headers_config: Option<::Value<CustomHeadersConfig>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html#cfn-cloudfront-responseheaderspolicy-responseheaderspolicyconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SecurityHeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html#cfn-cloudfront-responseheaderspolicy-responseheaderspolicyconfig-securityheadersconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_headers_config: Option<::Value<SecurityHeadersConfig>>,
    }

    impl ::codec::SerializeValue for ResponseHeadersPolicyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            if let Some(ref cors_config) = self.cors_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CorsConfig", cors_config)?;
            }
            if let Some(ref custom_headers_config) = self.custom_headers_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomHeadersConfig", custom_headers_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref security_headers_config) = self.security_headers_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityHeadersConfig", security_headers_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseHeadersPolicyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseHeadersPolicyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseHeadersPolicyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseHeadersPolicyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut cors_config: Option<::Value<CorsConfig>> = None;
                    let mut custom_headers_config: Option<::Value<CustomHeadersConfig>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut security_headers_config: Option<::Value<SecurityHeadersConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CorsConfig" => {
                                cors_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomHeadersConfig" => {
                                custom_headers_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityHeadersConfig" => {
                                security_headers_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseHeadersPolicyConfig {
                        comment: comment,
                        cors_config: cors_config,
                        custom_headers_config: custom_headers_config,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        security_headers_config: security_headers_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.SecurityHeadersConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SecurityHeadersConfig {
        /// Property [`ContentSecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html#cfn-cloudfront-responseheaderspolicy-securityheadersconfig-contentsecuritypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_security_policy: Option<::Value<ContentSecurityPolicy>>,
        /// Property [`ContentTypeOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html#cfn-cloudfront-responseheaderspolicy-securityheadersconfig-contenttypeoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type_options: Option<::Value<ContentTypeOptions>>,
        /// Property [`FrameOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html#cfn-cloudfront-responseheaderspolicy-securityheadersconfig-frameoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_options: Option<::Value<FrameOptions>>,
        /// Property [`ReferrerPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html#cfn-cloudfront-responseheaderspolicy-securityheadersconfig-referrerpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub referrer_policy: Option<::Value<ReferrerPolicy>>,
        /// Property [`StrictTransportSecurity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html#cfn-cloudfront-responseheaderspolicy-securityheadersconfig-stricttransportsecurity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strict_transport_security: Option<::Value<StrictTransportSecurity>>,
        /// Property [`XSSProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html#cfn-cloudfront-responseheaderspolicy-securityheadersconfig-xssprotection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub xss_protection: Option<::Value<XSSProtection>>,
    }

    impl ::codec::SerializeValue for SecurityHeadersConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_security_policy) = self.content_security_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentSecurityPolicy", content_security_policy)?;
            }
            if let Some(ref content_type_options) = self.content_type_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentTypeOptions", content_type_options)?;
            }
            if let Some(ref frame_options) = self.frame_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameOptions", frame_options)?;
            }
            if let Some(ref referrer_policy) = self.referrer_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferrerPolicy", referrer_policy)?;
            }
            if let Some(ref strict_transport_security) = self.strict_transport_security {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StrictTransportSecurity", strict_transport_security)?;
            }
            if let Some(ref xss_protection) = self.xss_protection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "XSSProtection", xss_protection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SecurityHeadersConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityHeadersConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SecurityHeadersConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SecurityHeadersConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_security_policy: Option<::Value<ContentSecurityPolicy>> = None;
                    let mut content_type_options: Option<::Value<ContentTypeOptions>> = None;
                    let mut frame_options: Option<::Value<FrameOptions>> = None;
                    let mut referrer_policy: Option<::Value<ReferrerPolicy>> = None;
                    let mut strict_transport_security: Option<::Value<StrictTransportSecurity>> = None;
                    let mut xss_protection: Option<::Value<XSSProtection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentSecurityPolicy" => {
                                content_security_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentTypeOptions" => {
                                content_type_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameOptions" => {
                                frame_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReferrerPolicy" => {
                                referrer_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StrictTransportSecurity" => {
                                strict_transport_security = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "XSSProtection" => {
                                xss_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SecurityHeadersConfig {
                        content_security_policy: content_security_policy,
                        content_type_options: content_type_options,
                        frame_options: frame_options,
                        referrer_policy: referrer_policy,
                        strict_transport_security: strict_transport_security,
                        xss_protection: xss_protection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.StrictTransportSecurity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-stricttransportsecurity.html) property type.
    #[derive(Debug, Default)]
    pub struct StrictTransportSecurity {
        /// Property [`AccessControlMaxAgeSec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-stricttransportsecurity.html#cfn-cloudfront-responseheaderspolicy-stricttransportsecurity-accesscontrolmaxagesec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_max_age_sec: ::Value<u32>,
        /// Property [`IncludeSubdomains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-stricttransportsecurity.html#cfn-cloudfront-responseheaderspolicy-stricttransportsecurity-includesubdomains).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_subdomains: Option<::Value<bool>>,
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-stricttransportsecurity.html#cfn-cloudfront-responseheaderspolicy-stricttransportsecurity-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
        /// Property [`Preload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-stricttransportsecurity.html#cfn-cloudfront-responseheaderspolicy-stricttransportsecurity-preload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preload: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for StrictTransportSecurity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlMaxAgeSec", &self.access_control_max_age_sec)?;
            if let Some(ref include_subdomains) = self.include_subdomains {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSubdomains", include_subdomains)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            if let Some(ref preload) = self.preload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Preload", preload)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StrictTransportSecurity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StrictTransportSecurity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StrictTransportSecurity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StrictTransportSecurity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_control_max_age_sec: Option<::Value<u32>> = None;
                    let mut include_subdomains: Option<::Value<bool>> = None;
                    let mut r#override: Option<::Value<bool>> = None;
                    let mut preload: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessControlMaxAgeSec" => {
                                access_control_max_age_sec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSubdomains" => {
                                include_subdomains = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Preload" => {
                                preload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StrictTransportSecurity {
                        access_control_max_age_sec: access_control_max_age_sec.ok_or(::serde::de::Error::missing_field("AccessControlMaxAgeSec"))?,
                        include_subdomains: include_subdomains,
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                        preload: preload,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::ResponseHeadersPolicy.XSSProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-xssprotection.html) property type.
    #[derive(Debug, Default)]
    pub struct XSSProtection {
        /// Property [`ModeBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-xssprotection.html#cfn-cloudfront-responseheaderspolicy-xssprotection-modeblock).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode_block: Option<::Value<bool>>,
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-xssprotection.html#cfn-cloudfront-responseheaderspolicy-xssprotection-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: ::Value<bool>,
        /// Property [`Protection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-xssprotection.html#cfn-cloudfront-responseheaderspolicy-xssprotection-protection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protection: ::Value<bool>,
        /// Property [`ReportUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-xssprotection.html#cfn-cloudfront-responseheaderspolicy-xssprotection-reporturi).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for XSSProtection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mode_block) = self.mode_block {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModeBlock", mode_block)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", &self.r#override)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protection", &self.protection)?;
            if let Some(ref report_uri) = self.report_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportUri", report_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for XSSProtection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<XSSProtection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = XSSProtection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type XSSProtection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode_block: Option<::Value<bool>> = None;
                    let mut r#override: Option<::Value<bool>> = None;
                    let mut protection: Option<::Value<bool>> = None;
                    let mut report_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ModeBlock" => {
                                mode_block = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protection" => {
                                protection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReportUri" => {
                                report_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(XSSProtection {
                        mode_block: mode_block,
                        r#override: r#override.ok_or(::serde::de::Error::missing_field("Override"))?,
                        protection: protection.ok_or(::serde::de::Error::missing_field("Protection"))?,
                        report_uri: report_uri,
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
    #[derive(Debug, Default)]
    pub struct Logging {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html#cfn-cloudfront-streamingdistribution-logging-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html#cfn-cloudfront-streamingdistribution-logging-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html#cfn-cloudfront-streamingdistribution-logging-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct S3Origin {
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html#cfn-cloudfront-streamingdistribution-s3origin-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: ::Value<String>,
        /// Property [`OriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html#cfn-cloudfront-streamingdistribution-s3origin-originaccessidentity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct StreamingDistributionConfig {
        /// Property [`Aliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-aliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aliases: Option<::ValueList<String>>,
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: ::Value<String>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<::Value<Logging>>,
        /// Property [`PriceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-priceclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub price_class: Option<::Value<String>>,
        /// Property [`S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-s3origin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_origin: ::Value<S3Origin>,
        /// Property [`TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html#cfn-cloudfront-streamingdistribution-streamingdistributionconfig-trustedsigners).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct TrustedSigners {
        /// Property [`AwsAccountNumbers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html#cfn-cloudfront-streamingdistribution-trustedsigners-awsaccountnumbers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_account_numbers: Option<::ValueList<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html#cfn-cloudfront-streamingdistribution-trustedsigners-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
