//! Types for the `S3ObjectLambda` service.

/// The [`AWS::S3ObjectLambda::AccessPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspoint.html) resource type.
#[derive(Debug, Default)]
pub struct AccessPoint {
    properties: AccessPointProperties
}

/// Properties for the `AccessPoint` resource.
#[derive(Debug, Default)]
pub struct AccessPointProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspoint.html#cfn-s3objectlambda-accesspoint-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ObjectLambdaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspoint.html#cfn-s3objectlambda-accesspoint-objectlambdaconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub object_lambda_configuration: Option<::Value<self::access_point::ObjectLambdaConfiguration>>,
}

impl ::serde::Serialize for AccessPointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref object_lambda_configuration) = self.object_lambda_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectLambdaConfiguration", object_lambda_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccessPointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccessPointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccessPointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut object_lambda_configuration: Option<::Value<self::access_point::ObjectLambdaConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectLambdaConfiguration" => {
                            object_lambda_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccessPointProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    object_lambda_configuration: object_lambda_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccessPoint {
    type Properties = AccessPointProperties;
    const TYPE: &'static str = "AWS::S3ObjectLambda::AccessPoint";
    fn properties(&self) -> &AccessPointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccessPointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccessPoint {}

impl From<AccessPointProperties> for AccessPoint {
    fn from(properties: AccessPointProperties) -> AccessPoint {
        AccessPoint { properties }
    }
}

/// The [`AWS::S3ObjectLambda::AccessPointPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspointpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct AccessPointPolicy {
    properties: AccessPointPolicyProperties
}

/// Properties for the `AccessPointPolicy` resource.
#[derive(Debug, Default)]
pub struct AccessPointPolicyProperties {
    /// Property [`ObjectLambdaAccessPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspointpolicy.html#cfn-s3objectlambda-accesspointpolicy-objectlambdaaccesspoint).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub object_lambda_access_point: ::Value<String>,
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspointpolicy.html#cfn-s3objectlambda-accesspointpolicy-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
}

impl ::serde::Serialize for AccessPointPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectLambdaAccessPoint", &self.object_lambda_access_point)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccessPointPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPointPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccessPointPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccessPointPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut object_lambda_access_point: Option<::Value<String>> = None;
                let mut policy_document: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ObjectLambdaAccessPoint" => {
                            object_lambda_access_point = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccessPointPolicyProperties {
                    object_lambda_access_point: object_lambda_access_point.ok_or(::serde::de::Error::missing_field("ObjectLambdaAccessPoint"))?,
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccessPointPolicy {
    type Properties = AccessPointPolicyProperties;
    const TYPE: &'static str = "AWS::S3ObjectLambda::AccessPointPolicy";
    fn properties(&self) -> &AccessPointPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccessPointPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccessPointPolicy {}

impl From<AccessPointPolicyProperties> for AccessPointPolicy {
    fn from(properties: AccessPointPolicyProperties) -> AccessPointPolicy {
        AccessPointPolicy { properties }
    }
}

pub mod access_point {
    //! Property types for the `AccessPoint` resource.

    /// The [`AWS::S3ObjectLambda::AccessPoint.ObjectLambdaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-objectlambdaconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ObjectLambdaConfiguration {
        /// Property [`AllowedFeatures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-objectlambdaconfiguration.html#cfn-s3objectlambda-accesspoint-objectlambdaconfiguration-allowedfeatures).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_features: Option<::ValueList<String>>,
        /// Property [`CloudWatchMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-objectlambdaconfiguration.html#cfn-s3objectlambda-accesspoint-objectlambdaconfiguration-cloudwatchmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_metrics_enabled: Option<::Value<bool>>,
        /// Property [`SupportingAccessPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-objectlambdaconfiguration.html#cfn-s3objectlambda-accesspoint-objectlambdaconfiguration-supportingaccesspoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supporting_access_point: ::Value<String>,
        /// Property [`TransformationConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-objectlambdaconfiguration.html#cfn-s3objectlambda-accesspoint-objectlambdaconfiguration-transformationconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transformation_configurations: ::ValueList<TransformationConfiguration>,
    }

    impl ::codec::SerializeValue for ObjectLambdaConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_features) = self.allowed_features {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedFeatures", allowed_features)?;
            }
            if let Some(ref cloud_watch_metrics_enabled) = self.cloud_watch_metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchMetricsEnabled", cloud_watch_metrics_enabled)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportingAccessPoint", &self.supporting_access_point)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformationConfigurations", &self.transformation_configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObjectLambdaConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectLambdaConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObjectLambdaConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObjectLambdaConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_features: Option<::ValueList<String>> = None;
                    let mut cloud_watch_metrics_enabled: Option<::Value<bool>> = None;
                    let mut supporting_access_point: Option<::Value<String>> = None;
                    let mut transformation_configurations: Option<::ValueList<TransformationConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedFeatures" => {
                                allowed_features = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchMetricsEnabled" => {
                                cloud_watch_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportingAccessPoint" => {
                                supporting_access_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransformationConfigurations" => {
                                transformation_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObjectLambdaConfiguration {
                        allowed_features: allowed_features,
                        cloud_watch_metrics_enabled: cloud_watch_metrics_enabled,
                        supporting_access_point: supporting_access_point.ok_or(::serde::de::Error::missing_field("SupportingAccessPoint"))?,
                        transformation_configurations: transformation_configurations.ok_or(::serde::de::Error::missing_field("TransformationConfigurations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::S3ObjectLambda::AccessPoint.TransformationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-transformationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformationConfiguration {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-transformationconfiguration.html#cfn-s3objectlambda-accesspoint-transformationconfiguration-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: Option<::ValueList<String>>,
        /// Property [`ContentTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-transformationconfiguration.html#cfn-s3objectlambda-accesspoint-transformationconfiguration-contenttransformation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_transformation: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for TransformationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref actions) = self.actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
            }
            if let Some(ref content_transformation) = self.content_transformation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentTransformation", content_transformation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut content_transformation: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentTransformation" => {
                                content_transformation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformationConfiguration {
                        actions: actions,
                        content_transformation: content_transformation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
