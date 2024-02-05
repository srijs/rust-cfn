//! Types for the `DevOpsGuru` service.

/// The [`AWS::DevOpsGuru::LogAnomalyDetectionIntegration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-loganomalydetectionintegration.html) resource type.
#[derive(Debug, Default)]
pub struct LogAnomalyDetectionIntegration {
    properties: LogAnomalyDetectionIntegrationProperties
}

/// Properties for the `LogAnomalyDetectionIntegration` resource.
#[derive(Debug, Default)]
pub struct LogAnomalyDetectionIntegrationProperties {
}

impl ::serde::Serialize for LogAnomalyDetectionIntegrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LogAnomalyDetectionIntegrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LogAnomalyDetectionIntegrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LogAnomalyDetectionIntegrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LogAnomalyDetectionIntegrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                Ok(LogAnomalyDetectionIntegrationProperties {})
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LogAnomalyDetectionIntegration {
    type Properties = LogAnomalyDetectionIntegrationProperties;
    const TYPE: &'static str = "AWS::DevOpsGuru::LogAnomalyDetectionIntegration";
    fn properties(&self) -> &LogAnomalyDetectionIntegrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LogAnomalyDetectionIntegrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LogAnomalyDetectionIntegration {}

impl From<LogAnomalyDetectionIntegrationProperties> for LogAnomalyDetectionIntegration {
    fn from(properties: LogAnomalyDetectionIntegrationProperties) -> LogAnomalyDetectionIntegration {
        LogAnomalyDetectionIntegration { properties }
    }
}

/// The [`AWS::DevOpsGuru::NotificationChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-notificationchannel.html) resource type.
#[derive(Debug, Default)]
pub struct NotificationChannel {
    properties: NotificationChannelProperties
}

/// Properties for the `NotificationChannel` resource.
#[derive(Debug, Default)]
pub struct NotificationChannelProperties {
    /// Property [`Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-notificationchannel.html#cfn-devopsguru-notificationchannel-config).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub config: ::Value<self::notification_channel::NotificationChannelConfig>,
}

impl ::serde::Serialize for NotificationChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Config", &self.config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NotificationChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NotificationChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NotificationChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config: Option<::Value<self::notification_channel::NotificationChannelConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Config" => {
                            config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NotificationChannelProperties {
                    config: config.ok_or(::serde::de::Error::missing_field("Config"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NotificationChannel {
    type Properties = NotificationChannelProperties;
    const TYPE: &'static str = "AWS::DevOpsGuru::NotificationChannel";
    fn properties(&self) -> &NotificationChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NotificationChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NotificationChannel {}

impl From<NotificationChannelProperties> for NotificationChannel {
    fn from(properties: NotificationChannelProperties) -> NotificationChannel {
        NotificationChannel { properties }
    }
}

/// The [`AWS::DevOpsGuru::ResourceCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-resourcecollection.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceCollection {
    properties: ResourceCollectionProperties
}

/// Properties for the `ResourceCollection` resource.
#[derive(Debug, Default)]
pub struct ResourceCollectionProperties {
    /// Property [`ResourceCollectionFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-resourcecollection.html#cfn-devopsguru-resourcecollection-resourcecollectionfilter).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_collection_filter: ::Value<self::resource_collection::ResourceCollectionFilter>,
}

impl ::serde::Serialize for ResourceCollectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceCollectionFilter", &self.resource_collection_filter)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceCollectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceCollectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceCollectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceCollectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_collection_filter: Option<::Value<self::resource_collection::ResourceCollectionFilter>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceCollectionFilter" => {
                            resource_collection_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceCollectionProperties {
                    resource_collection_filter: resource_collection_filter.ok_or(::serde::de::Error::missing_field("ResourceCollectionFilter"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceCollection {
    type Properties = ResourceCollectionProperties;
    const TYPE: &'static str = "AWS::DevOpsGuru::ResourceCollection";
    fn properties(&self) -> &ResourceCollectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceCollectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceCollection {}

impl From<ResourceCollectionProperties> for ResourceCollection {
    fn from(properties: ResourceCollectionProperties) -> ResourceCollection {
        ResourceCollection { properties }
    }
}

pub mod notification_channel {
    //! Property types for the `NotificationChannel` resource.

    /// The [`AWS::DevOpsGuru::NotificationChannel.NotificationChannelConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationchannelconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationChannelConfig {
        /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationchannelconfig.html#cfn-devopsguru-notificationchannel-notificationchannelconfig-filters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub filters: Option<::Value<NotificationFilterConfig>>,
        /// Property [`Sns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationchannelconfig.html#cfn-devopsguru-notificationchannel-notificationchannelconfig-sns).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sns: Option<::Value<SnsChannelConfig>>,
    }

    impl ::codec::SerializeValue for NotificationChannelConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref filters) = self.filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filters", filters)?;
            }
            if let Some(ref sns) = self.sns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sns", sns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationChannelConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationChannelConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationChannelConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationChannelConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filters: Option<::Value<NotificationFilterConfig>> = None;
                    let mut sns: Option<::Value<SnsChannelConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Filters" => {
                                filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sns" => {
                                sns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationChannelConfig {
                        filters: filters,
                        sns: sns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DevOpsGuru::NotificationChannel.NotificationFilterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationfilterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationFilterConfig {
        /// Property [`MessageTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationfilterconfig.html#cfn-devopsguru-notificationchannel-notificationfilterconfig-messagetypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub message_types: Option<::ValueList<String>>,
        /// Property [`Severities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationfilterconfig.html#cfn-devopsguru-notificationchannel-notificationfilterconfig-severities).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub severities: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for NotificationFilterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message_types) = self.message_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageTypes", message_types)?;
            }
            if let Some(ref severities) = self.severities {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Severities", severities)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationFilterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationFilterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationFilterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationFilterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_types: Option<::ValueList<String>> = None;
                    let mut severities: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageTypes" => {
                                message_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Severities" => {
                                severities = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationFilterConfig {
                        message_types: message_types,
                        severities: severities,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DevOpsGuru::NotificationChannel.SnsChannelConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-snschannelconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SnsChannelConfig {
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-snschannelconfig.html#cfn-devopsguru-notificationchannel-snschannelconfig-topicarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SnsChannelConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnsChannelConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnsChannelConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnsChannelConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnsChannelConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnsChannelConfig {
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resource_collection {
    //! Property types for the `ResourceCollection` resource.

    /// The [`AWS::DevOpsGuru::ResourceCollection.CloudFormationCollectionFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-cloudformationcollectionfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudFormationCollectionFilter {
        /// Property [`StackNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-cloudformationcollectionfilter.html#cfn-devopsguru-resourcecollection-cloudformationcollectionfilter-stacknames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CloudFormationCollectionFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref stack_names) = self.stack_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackNames", stack_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudFormationCollectionFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFormationCollectionFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudFormationCollectionFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudFormationCollectionFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stack_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StackNames" => {
                                stack_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudFormationCollectionFilter {
                        stack_names: stack_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DevOpsGuru::ResourceCollection.ResourceCollectionFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-resourcecollectionfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceCollectionFilter {
        /// Property [`CloudFormation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-resourcecollectionfilter.html#cfn-devopsguru-resourcecollection-resourcecollectionfilter-cloudformation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_formation: Option<::Value<CloudFormationCollectionFilter>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-resourcecollectionfilter.html#cfn-devopsguru-resourcecollection-resourcecollectionfilter-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<TagCollection>>,
    }

    impl ::codec::SerializeValue for ResourceCollectionFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_formation) = self.cloud_formation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudFormation", cloud_formation)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceCollectionFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceCollectionFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceCollectionFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceCollectionFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_formation: Option<::Value<CloudFormationCollectionFilter>> = None;
                    let mut tags: Option<::ValueList<TagCollection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudFormation" => {
                                cloud_formation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceCollectionFilter {
                        cloud_formation: cloud_formation,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DevOpsGuru::ResourceCollection.TagCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-tagcollection.html) property type.
    #[derive(Debug, Default)]
    pub struct TagCollection {
        /// Property [`AppBoundaryKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-tagcollection.html#cfn-devopsguru-resourcecollection-tagcollection-appboundarykey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_boundary_key: Option<::Value<String>>,
        /// Property [`TagValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-tagcollection.html#cfn-devopsguru-resourcecollection-tagcollection-tagvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for TagCollection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_boundary_key) = self.app_boundary_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppBoundaryKey", app_boundary_key)?;
            }
            if let Some(ref tag_values) = self.tag_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValues", tag_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagCollection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagCollection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagCollection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagCollection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_boundary_key: Option<::Value<String>> = None;
                    let mut tag_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppBoundaryKey" => {
                                app_boundary_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValues" => {
                                tag_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagCollection {
                        app_boundary_key: app_boundary_key,
                        tag_values: tag_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
