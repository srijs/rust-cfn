//! Types for the `InternetMonitor` service.

/// The [`AWS::InternetMonitor::Monitor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html) resource type.
#[derive(Debug, Default)]
pub struct Monitor {
    properties: MonitorProperties
}

/// Properties for the `Monitor` resource.
#[derive(Debug, Default)]
pub struct MonitorProperties {
    /// Property [`HealthEventsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-healtheventsconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_events_config: Option<::Value<self::monitor::HealthEventsConfig>>,
    /// Property [`InternetMeasurementsLogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-internetmeasurementslogdelivery).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub internet_measurements_log_delivery: Option<::Value<self::monitor::InternetMeasurementsLogDelivery>>,
    /// Property [`MaxCityNetworksToMonitor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-maxcitynetworkstomonitor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_city_networks_to_monitor: Option<::Value<u32>>,
    /// Property [`MonitorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-monitorname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub monitor_name: ::Value<String>,
    /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-resources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources: Option<::ValueList<String>>,
    /// Property [`ResourcesToAdd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-resourcestoadd).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources_to_add: Option<::ValueList<String>>,
    /// Property [`ResourcesToRemove`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-resourcestoremove).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources_to_remove: Option<::ValueList<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TrafficPercentageToMonitor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html#cfn-internetmonitor-monitor-trafficpercentagetomonitor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub traffic_percentage_to_monitor: Option<::Value<u32>>,
}

impl ::serde::Serialize for MonitorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref health_events_config) = self.health_events_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthEventsConfig", health_events_config)?;
        }
        if let Some(ref internet_measurements_log_delivery) = self.internet_measurements_log_delivery {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InternetMeasurementsLogDelivery", internet_measurements_log_delivery)?;
        }
        if let Some(ref max_city_networks_to_monitor) = self.max_city_networks_to_monitor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCityNetworksToMonitor", max_city_networks_to_monitor)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitorName", &self.monitor_name)?;
        if let Some(ref resources) = self.resources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", resources)?;
        }
        if let Some(ref resources_to_add) = self.resources_to_add {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcesToAdd", resources_to_add)?;
        }
        if let Some(ref resources_to_remove) = self.resources_to_remove {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcesToRemove", resources_to_remove)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref traffic_percentage_to_monitor) = self.traffic_percentage_to_monitor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrafficPercentageToMonitor", traffic_percentage_to_monitor)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MonitorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MonitorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MonitorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut health_events_config: Option<::Value<self::monitor::HealthEventsConfig>> = None;
                let mut internet_measurements_log_delivery: Option<::Value<self::monitor::InternetMeasurementsLogDelivery>> = None;
                let mut max_city_networks_to_monitor: Option<::Value<u32>> = None;
                let mut monitor_name: Option<::Value<String>> = None;
                let mut resources: Option<::ValueList<String>> = None;
                let mut resources_to_add: Option<::ValueList<String>> = None;
                let mut resources_to_remove: Option<::ValueList<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut traffic_percentage_to_monitor: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HealthEventsConfig" => {
                            health_events_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InternetMeasurementsLogDelivery" => {
                            internet_measurements_log_delivery = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxCityNetworksToMonitor" => {
                            max_city_networks_to_monitor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitorName" => {
                            monitor_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resources" => {
                            resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcesToAdd" => {
                            resources_to_add = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcesToRemove" => {
                            resources_to_remove = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrafficPercentageToMonitor" => {
                            traffic_percentage_to_monitor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MonitorProperties {
                    health_events_config: health_events_config,
                    internet_measurements_log_delivery: internet_measurements_log_delivery,
                    max_city_networks_to_monitor: max_city_networks_to_monitor,
                    monitor_name: monitor_name.ok_or(::serde::de::Error::missing_field("MonitorName"))?,
                    resources: resources,
                    resources_to_add: resources_to_add,
                    resources_to_remove: resources_to_remove,
                    status: status,
                    tags: tags,
                    traffic_percentage_to_monitor: traffic_percentage_to_monitor,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Monitor {
    type Properties = MonitorProperties;
    const TYPE: &'static str = "AWS::InternetMonitor::Monitor";
    fn properties(&self) -> &MonitorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MonitorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Monitor {}

impl From<MonitorProperties> for Monitor {
    fn from(properties: MonitorProperties) -> Monitor {
        Monitor { properties }
    }
}

pub mod monitor {
    //! Property types for the `Monitor` resource.

    /// The [`AWS::InternetMonitor::Monitor.HealthEventsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-healtheventsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthEventsConfig {
        /// Property [`AvailabilityLocalHealthEventsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-healtheventsconfig.html#cfn-internetmonitor-monitor-healtheventsconfig-availabilitylocalhealtheventsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_local_health_events_config: Option<::Value<LocalHealthEventsConfig>>,
        /// Property [`AvailabilityScoreThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-healtheventsconfig.html#cfn-internetmonitor-monitor-healtheventsconfig-availabilityscorethreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_score_threshold: Option<::Value<f64>>,
        /// Property [`PerformanceLocalHealthEventsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-healtheventsconfig.html#cfn-internetmonitor-monitor-healtheventsconfig-performancelocalhealtheventsconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub performance_local_health_events_config: Option<::Value<LocalHealthEventsConfig>>,
        /// Property [`PerformanceScoreThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-healtheventsconfig.html#cfn-internetmonitor-monitor-healtheventsconfig-performancescorethreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub performance_score_threshold: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for HealthEventsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_local_health_events_config) = self.availability_local_health_events_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityLocalHealthEventsConfig", availability_local_health_events_config)?;
            }
            if let Some(ref availability_score_threshold) = self.availability_score_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityScoreThreshold", availability_score_threshold)?;
            }
            if let Some(ref performance_local_health_events_config) = self.performance_local_health_events_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceLocalHealthEventsConfig", performance_local_health_events_config)?;
            }
            if let Some(ref performance_score_threshold) = self.performance_score_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceScoreThreshold", performance_score_threshold)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthEventsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthEventsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthEventsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthEventsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_local_health_events_config: Option<::Value<LocalHealthEventsConfig>> = None;
                    let mut availability_score_threshold: Option<::Value<f64>> = None;
                    let mut performance_local_health_events_config: Option<::Value<LocalHealthEventsConfig>> = None;
                    let mut performance_score_threshold: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityLocalHealthEventsConfig" => {
                                availability_local_health_events_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AvailabilityScoreThreshold" => {
                                availability_score_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerformanceLocalHealthEventsConfig" => {
                                performance_local_health_events_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerformanceScoreThreshold" => {
                                performance_score_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthEventsConfig {
                        availability_local_health_events_config: availability_local_health_events_config,
                        availability_score_threshold: availability_score_threshold,
                        performance_local_health_events_config: performance_local_health_events_config,
                        performance_score_threshold: performance_score_threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InternetMonitor::Monitor.InternetMeasurementsLogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-internetmeasurementslogdelivery.html) property type.
    #[derive(Debug, Default)]
    pub struct InternetMeasurementsLogDelivery {
        /// Property [`S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-internetmeasurementslogdelivery.html#cfn-internetmonitor-monitor-internetmeasurementslogdelivery-s3config).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_config: Option<::Value<S3Config>>,
    }

    impl ::codec::SerializeValue for InternetMeasurementsLogDelivery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_config) = self.s3_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Config", s3_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InternetMeasurementsLogDelivery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InternetMeasurementsLogDelivery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InternetMeasurementsLogDelivery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InternetMeasurementsLogDelivery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_config: Option<::Value<S3Config>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Config" => {
                                s3_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InternetMeasurementsLogDelivery {
                        s3_config: s3_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InternetMonitor::Monitor.LocalHealthEventsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-localhealtheventsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalHealthEventsConfig {
        /// Property [`HealthScoreThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-localhealtheventsconfig.html#cfn-internetmonitor-monitor-localhealtheventsconfig-healthscorethreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub health_score_threshold: Option<::Value<f64>>,
        /// Property [`MinTrafficImpact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-localhealtheventsconfig.html#cfn-internetmonitor-monitor-localhealtheventsconfig-mintrafficimpact).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_traffic_impact: Option<::Value<f64>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-localhealtheventsconfig.html#cfn-internetmonitor-monitor-localhealtheventsconfig-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LocalHealthEventsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref health_score_threshold) = self.health_score_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthScoreThreshold", health_score_threshold)?;
            }
            if let Some(ref min_traffic_impact) = self.min_traffic_impact {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTrafficImpact", min_traffic_impact)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalHealthEventsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalHealthEventsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalHealthEventsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalHealthEventsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut health_score_threshold: Option<::Value<f64>> = None;
                    let mut min_traffic_impact: Option<::Value<f64>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthScoreThreshold" => {
                                health_score_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTrafficImpact" => {
                                min_traffic_impact = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalHealthEventsConfig {
                        health_score_threshold: health_score_threshold,
                        min_traffic_impact: min_traffic_impact,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InternetMonitor::Monitor.S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-s3config.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Config {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-s3config.html#cfn-internetmonitor-monitor-s3config-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-s3config.html#cfn-internetmonitor-monitor-s3config-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`LogDeliveryStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-s3config.html#cfn-internetmonitor-monitor-s3config-logdeliverystatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_delivery_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref log_delivery_status) = self.log_delivery_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDeliveryStatus", log_delivery_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut log_delivery_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogDeliveryStatus" => {
                                log_delivery_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Config {
                        bucket_name: bucket_name,
                        bucket_prefix: bucket_prefix,
                        log_delivery_status: log_delivery_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
