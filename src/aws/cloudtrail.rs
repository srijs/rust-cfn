//! Types for the `CloudTrail` service.

/// The [`AWS::CloudTrail::Trail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html) resource type.
#[derive(Debug)]
pub struct Trail {
    properties: TrailProperties
}

/// Properties for the `Trail` resource.
#[derive(Debug)]
pub struct TrailProperties {
    /// Property `CloudWatchLogsLogGroupArn`.
    pub cloud_watch_logs_log_group_arn: Option<::Value<String>>,
    /// Property `CloudWatchLogsRoleArn`.
    pub cloud_watch_logs_role_arn: Option<::Value<String>>,
    /// Property `EnableLogFileValidation`.
    pub enable_log_file_validation: Option<::Value<bool>>,
    /// Property `EventSelectors`.
    pub event_selectors: Option<::ValueList<self::trail::EventSelector>>,
    /// Property `IncludeGlobalServiceEvents`.
    pub include_global_service_events: Option<::Value<bool>>,
    /// Property `IsLogging`.
    pub is_logging: ::Value<bool>,
    /// Property `IsMultiRegionTrail`.
    pub is_multi_region_trail: Option<::Value<bool>>,
    /// Property `KMSKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `S3BucketName`.
    pub s3_bucket_name: ::Value<String>,
    /// Property `S3KeyPrefix`.
    pub s3_key_prefix: Option<::Value<String>>,
    /// Property `SnsTopicName`.
    pub sns_topic_name: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TrailName`.
    pub trail_name: Option<::Value<String>>,
}

impl ::serde::Serialize for TrailProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cloud_watch_logs_log_group_arn) = self.cloud_watch_logs_log_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsLogGroupArn", cloud_watch_logs_log_group_arn)?;
        }
        if let Some(ref cloud_watch_logs_role_arn) = self.cloud_watch_logs_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsRoleArn", cloud_watch_logs_role_arn)?;
        }
        if let Some(ref enable_log_file_validation) = self.enable_log_file_validation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableLogFileValidation", enable_log_file_validation)?;
        }
        if let Some(ref event_selectors) = self.event_selectors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSelectors", event_selectors)?;
        }
        if let Some(ref include_global_service_events) = self.include_global_service_events {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeGlobalServiceEvents", include_global_service_events)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsLogging", &self.is_logging)?;
        if let Some(ref is_multi_region_trail) = self.is_multi_region_trail {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsMultiRegionTrail", is_multi_region_trail)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
        if let Some(ref s3_key_prefix) = self.s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
        }
        if let Some(ref sns_topic_name) = self.sns_topic_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicName", sns_topic_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref trail_name) = self.trail_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrailName", trail_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrailProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrailProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrailProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrailProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cloud_watch_logs_log_group_arn = None;
                let mut cloud_watch_logs_role_arn = None;
                let mut enable_log_file_validation = None;
                let mut event_selectors = None;
                let mut include_global_service_events = None;
                let mut is_logging = None;
                let mut is_multi_region_trail = None;
                let mut kms_key_id = None;
                let mut s3_bucket_name = None;
                let mut s3_key_prefix = None;
                let mut sns_topic_name = None;
                let mut tags = None;
                let mut trail_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudWatchLogsLogGroupArn" => {
                            cloud_watch_logs_log_group_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CloudWatchLogsRoleArn" => {
                            cloud_watch_logs_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnableLogFileValidation" => {
                            enable_log_file_validation = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EventSelectors" => {
                            event_selectors = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IncludeGlobalServiceEvents" => {
                            include_global_service_events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IsLogging" => {
                            is_logging = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IsMultiRegionTrail" => {
                            is_multi_region_trail = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KMSKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "S3BucketName" => {
                            s3_bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "S3KeyPrefix" => {
                            s3_key_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnsTopicName" => {
                            sns_topic_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TrailName" => {
                            trail_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(TrailProperties {
                    cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn,
                    cloud_watch_logs_role_arn: cloud_watch_logs_role_arn,
                    enable_log_file_validation: enable_log_file_validation,
                    event_selectors: event_selectors,
                    include_global_service_events: include_global_service_events,
                    is_logging: is_logging.ok_or(::serde::de::Error::missing_field("IsLogging"))?,
                    is_multi_region_trail: is_multi_region_trail,
                    kms_key_id: kms_key_id,
                    s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                    s3_key_prefix: s3_key_prefix,
                    sns_topic_name: sns_topic_name,
                    tags: tags,
                    trail_name: trail_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Trail {
    type Properties = TrailProperties;
    const TYPE: &'static str = "AWS::CloudTrail::Trail";
    fn properties(&self) -> &TrailProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrailProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Trail {}

impl From<TrailProperties> for Trail {
    fn from(properties: TrailProperties) -> Trail {
        Trail { properties }
    }
}

pub mod trail {
    //! Property types for the `Trail` resource.

    /// The [`AWS::CloudTrail::Trail.DataResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html) property type.
    #[derive(Debug)]
    pub struct DataResource {
        /// Property `Type`.
        pub type_: ::Value<String>,
        /// Property `Values`.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for DataResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_ = None;
                    let mut values = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Values" => {
                                values = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DataResource {
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudTrail::Trail.EventSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html) property type.
    #[derive(Debug)]
    pub struct EventSelector {
        /// Property `DataResources`.
        pub data_resources: Option<::ValueList<DataResource>>,
        /// Property `IncludeManagementEvents`.
        pub include_management_events: Option<::Value<bool>>,
        /// Property `ReadWriteType`.
        pub read_write_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_resources) = self.data_resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataResources", data_resources)?;
            }
            if let Some(ref include_management_events) = self.include_management_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeManagementEvents", include_management_events)?;
            }
            if let Some(ref read_write_type) = self.read_write_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadWriteType", read_write_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_resources = None;
                    let mut include_management_events = None;
                    let mut read_write_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataResources" => {
                                data_resources = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IncludeManagementEvents" => {
                                include_management_events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReadWriteType" => {
                                read_write_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EventSelector {
                        data_resources: data_resources,
                        include_management_events: include_management_events,
                        read_write_type: read_write_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
