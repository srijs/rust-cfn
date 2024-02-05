//! Types for the `OSIS` service.

/// The [`AWS::OSIS::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html) resource type.
#[derive(Debug, Default)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug, Default)]
pub struct PipelineProperties {
    /// Property [`BufferOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-bufferoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub buffer_options: Option<::Value<self::pipeline::BufferOptions>>,
    /// Property [`EncryptionAtRestOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-encryptionatrestoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_at_rest_options: Option<::Value<self::pipeline::EncryptionAtRestOptions>>,
    /// Property [`LogPublishingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-logpublishingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_publishing_options: Option<::Value<self::pipeline::LogPublishingOptions>>,
    /// Property [`MaxUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-maxunits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_units: ::Value<u32>,
    /// Property [`MinUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-minunits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_units: ::Value<u32>,
    /// Property [`PipelineConfigurationBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-pipelineconfigurationbody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pipeline_configuration_body: ::Value<String>,
    /// Property [`PipelineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-pipelinename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pipeline_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html#cfn-osis-pipeline-vpcoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_options: Option<::Value<self::pipeline::VpcOptions>>,
}

impl ::serde::Serialize for PipelineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref buffer_options) = self.buffer_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferOptions", buffer_options)?;
        }
        if let Some(ref encryption_at_rest_options) = self.encryption_at_rest_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionAtRestOptions", encryption_at_rest_options)?;
        }
        if let Some(ref log_publishing_options) = self.log_publishing_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPublishingOptions", log_publishing_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxUnits", &self.max_units)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinUnits", &self.min_units)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineConfigurationBody", &self.pipeline_configuration_body)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineName", &self.pipeline_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_options) = self.vpc_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcOptions", vpc_options)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PipelineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PipelineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PipelineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut buffer_options: Option<::Value<self::pipeline::BufferOptions>> = None;
                let mut encryption_at_rest_options: Option<::Value<self::pipeline::EncryptionAtRestOptions>> = None;
                let mut log_publishing_options: Option<::Value<self::pipeline::LogPublishingOptions>> = None;
                let mut max_units: Option<::Value<u32>> = None;
                let mut min_units: Option<::Value<u32>> = None;
                let mut pipeline_configuration_body: Option<::Value<String>> = None;
                let mut pipeline_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_options: Option<::Value<self::pipeline::VpcOptions>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BufferOptions" => {
                            buffer_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionAtRestOptions" => {
                            encryption_at_rest_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogPublishingOptions" => {
                            log_publishing_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxUnits" => {
                            max_units = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinUnits" => {
                            min_units = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineConfigurationBody" => {
                            pipeline_configuration_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineName" => {
                            pipeline_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcOptions" => {
                            vpc_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PipelineProperties {
                    buffer_options: buffer_options,
                    encryption_at_rest_options: encryption_at_rest_options,
                    log_publishing_options: log_publishing_options,
                    max_units: max_units.ok_or(::serde::de::Error::missing_field("MaxUnits"))?,
                    min_units: min_units.ok_or(::serde::de::Error::missing_field("MinUnits"))?,
                    pipeline_configuration_body: pipeline_configuration_body.ok_or(::serde::de::Error::missing_field("PipelineConfigurationBody"))?,
                    pipeline_name: pipeline_name.ok_or(::serde::de::Error::missing_field("PipelineName"))?,
                    tags: tags,
                    vpc_options: vpc_options,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Pipeline {
    type Properties = PipelineProperties;
    const TYPE: &'static str = "AWS::OSIS::Pipeline";
    fn properties(&self) -> &PipelineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PipelineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Pipeline {}

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

pub mod pipeline {
    //! Property types for the `Pipeline` resource.

    /// The [`AWS::OSIS::Pipeline.BufferOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-bufferoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct BufferOptions {
        /// Property [`PersistentBufferEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-bufferoptions.html#cfn-osis-pipeline-bufferoptions-persistentbufferenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub persistent_buffer_enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for BufferOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PersistentBufferEnabled", &self.persistent_buffer_enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BufferOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BufferOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BufferOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BufferOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut persistent_buffer_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PersistentBufferEnabled" => {
                                persistent_buffer_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BufferOptions {
                        persistent_buffer_enabled: persistent_buffer_enabled.ok_or(::serde::de::Error::missing_field("PersistentBufferEnabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OSIS::Pipeline.CloudWatchLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-cloudwatchlogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogDestination {
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-cloudwatchlogdestination.html#cfn-osis-pipeline-cloudwatchlogdestination-loggroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", &self.log_group)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogDestination {
                        log_group: log_group.ok_or(::serde::de::Error::missing_field("LogGroup"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OSIS::Pipeline.EncryptionAtRestOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-encryptionatrestoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionAtRestOptions {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-encryptionatrestoptions.html#cfn-osis-pipeline-encryptionatrestoptions-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionAtRestOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", &self.kms_key_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionAtRestOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionAtRestOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionAtRestOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionAtRestOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionAtRestOptions {
                        kms_key_arn: kms_key_arn.ok_or(::serde::de::Error::missing_field("KmsKeyArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OSIS::Pipeline.LogPublishingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-logpublishingoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct LogPublishingOptions {
        /// Property [`CloudWatchLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-logpublishingoptions.html#cfn-osis-pipeline-logpublishingoptions-cloudwatchlogdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_log_destination: Option<::Value<CloudWatchLogDestination>>,
        /// Property [`IsLoggingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-logpublishingoptions.html#cfn-osis-pipeline-logpublishingoptions-isloggingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_logging_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LogPublishingOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_log_destination) = self.cloud_watch_log_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogDestination", cloud_watch_log_destination)?;
            }
            if let Some(ref is_logging_enabled) = self.is_logging_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsLoggingEnabled", is_logging_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogPublishingOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogPublishingOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogPublishingOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogPublishingOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_log_destination: Option<::Value<CloudWatchLogDestination>> = None;
                    let mut is_logging_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogDestination" => {
                                cloud_watch_log_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsLoggingEnabled" => {
                                is_logging_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogPublishingOptions {
                        cloud_watch_log_destination: cloud_watch_log_destination,
                        is_logging_enabled: is_logging_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OSIS::Pipeline.VpcEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcEndpoint {
        /// Property [`VpcEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcendpoint.html#cfn-osis-pipeline-vpcendpoint-vpcendpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_endpoint_id: Option<::Value<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcendpoint.html#cfn-osis-pipeline-vpcendpoint-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
        /// Property [`VpcOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcendpoint.html#cfn-osis-pipeline-vpcendpoint-vpcoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_options: Option<::Value<VpcOptions>>,
    }

    impl ::codec::SerializeValue for VpcEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref vpc_endpoint_id) = self.vpc_endpoint_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpointId", vpc_endpoint_id)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            if let Some(ref vpc_options) = self.vpc_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcOptions", vpc_options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut vpc_endpoint_id: Option<::Value<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;
                    let mut vpc_options: Option<::Value<VpcOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VpcEndpointId" => {
                                vpc_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcOptions" => {
                                vpc_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcEndpoint {
                        vpc_endpoint_id: vpc_endpoint_id,
                        vpc_id: vpc_id,
                        vpc_options: vpc_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OSIS::Pipeline.VpcOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcOptions {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcoptions.html#cfn-osis-pipeline-vpcoptions-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcoptions.html#cfn-osis-pipeline-vpcoptions-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcOptions {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
