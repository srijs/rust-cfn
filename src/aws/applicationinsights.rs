//! Types for the `ApplicationInsights` service.

/// The [`AWS::ApplicationInsights::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`AutoConfigurationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-autoconfigurationenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_configuration_enabled: Option<::Value<bool>>,
    /// Property [`CWEMonitorEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-cwemonitorenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cwe_monitor_enabled: Option<::Value<bool>>,
    /// Property [`ComponentMonitoringSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-componentmonitoringsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub component_monitoring_settings: Option<::ValueList<self::application::ComponentMonitoringSetting>>,
    /// Property [`CustomComponents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-customcomponents).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_components: Option<::ValueList<self::application::CustomComponent>>,
    /// Property [`GroupingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-groupingtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub grouping_type: Option<::Value<String>>,
    /// Property [`LogPatternSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-logpatternsets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_pattern_sets: Option<::ValueList<self::application::LogPatternSet>>,
    /// Property [`OpsCenterEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-opscenterenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ops_center_enabled: Option<::Value<bool>>,
    /// Property [`OpsItemSNSTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-opsitemsnstopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ops_item_sns_topic_arn: Option<::Value<String>>,
    /// Property [`ResourceGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-resourcegroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_group_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html#cfn-applicationinsights-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_configuration_enabled) = self.auto_configuration_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoConfigurationEnabled", auto_configuration_enabled)?;
        }
        if let Some(ref cwe_monitor_enabled) = self.cwe_monitor_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CWEMonitorEnabled", cwe_monitor_enabled)?;
        }
        if let Some(ref component_monitoring_settings) = self.component_monitoring_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentMonitoringSettings", component_monitoring_settings)?;
        }
        if let Some(ref custom_components) = self.custom_components {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomComponents", custom_components)?;
        }
        if let Some(ref grouping_type) = self.grouping_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupingType", grouping_type)?;
        }
        if let Some(ref log_pattern_sets) = self.log_pattern_sets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPatternSets", log_pattern_sets)?;
        }
        if let Some(ref ops_center_enabled) = self.ops_center_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpsCenterEnabled", ops_center_enabled)?;
        }
        if let Some(ref ops_item_sns_topic_arn) = self.ops_item_sns_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpsItemSNSTopicArn", ops_item_sns_topic_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceGroupName", &self.resource_group_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_configuration_enabled: Option<::Value<bool>> = None;
                let mut cwe_monitor_enabled: Option<::Value<bool>> = None;
                let mut component_monitoring_settings: Option<::ValueList<self::application::ComponentMonitoringSetting>> = None;
                let mut custom_components: Option<::ValueList<self::application::CustomComponent>> = None;
                let mut grouping_type: Option<::Value<String>> = None;
                let mut log_pattern_sets: Option<::ValueList<self::application::LogPatternSet>> = None;
                let mut ops_center_enabled: Option<::Value<bool>> = None;
                let mut ops_item_sns_topic_arn: Option<::Value<String>> = None;
                let mut resource_group_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoConfigurationEnabled" => {
                            auto_configuration_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CWEMonitorEnabled" => {
                            cwe_monitor_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComponentMonitoringSettings" => {
                            component_monitoring_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomComponents" => {
                            custom_components = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupingType" => {
                            grouping_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogPatternSets" => {
                            log_pattern_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpsCenterEnabled" => {
                            ops_center_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpsItemSNSTopicArn" => {
                            ops_item_sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceGroupName" => {
                            resource_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    auto_configuration_enabled: auto_configuration_enabled,
                    cwe_monitor_enabled: cwe_monitor_enabled,
                    component_monitoring_settings: component_monitoring_settings,
                    custom_components: custom_components,
                    grouping_type: grouping_type,
                    log_pattern_sets: log_pattern_sets,
                    ops_center_enabled: ops_center_enabled,
                    ops_item_sns_topic_arn: ops_item_sns_topic_arn,
                    resource_group_name: resource_group_name.ok_or(::serde::de::Error::missing_field("ResourceGroupName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::ApplicationInsights::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::ApplicationInsights::Application.Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarm.html) property type.
    #[derive(Debug, Default)]
    pub struct Alarm {
        /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarm.html#cfn-applicationinsights-application-alarm-alarmname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_name: ::Value<String>,
        /// Property [`Severity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarm.html#cfn-applicationinsights-application-alarm-severity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub severity: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Alarm {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
            if let Some(ref severity) = self.severity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Severity", severity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Alarm {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Alarm, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Alarm;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Alarm")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_name: Option<::Value<String>> = None;
                    let mut severity: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmName" => {
                                alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Severity" => {
                                severity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Alarm {
                        alarm_name: alarm_name.ok_or(::serde::de::Error::missing_field("AlarmName"))?,
                        severity: severity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.AlarmMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarmmetric.html) property type.
    #[derive(Debug, Default)]
    pub struct AlarmMetric {
        /// Property [`AlarmMetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarmmetric.html#cfn-applicationinsights-application-alarmmetric-alarmmetricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_metric_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AlarmMetric {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmMetricName", &self.alarm_metric_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AlarmMetric {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmMetric, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AlarmMetric;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AlarmMetric")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_metric_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmMetricName" => {
                                alarm_metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AlarmMetric {
                        alarm_metric_name: alarm_metric_name.ok_or(::serde::de::Error::missing_field("AlarmMetricName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.ComponentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentConfiguration {
        /// Property [`ConfigurationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentconfiguration.html#cfn-applicationinsights-application-componentconfiguration-configurationdetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_details: Option<::Value<ConfigurationDetails>>,
        /// Property [`SubComponentTypeConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentconfiguration.html#cfn-applicationinsights-application-componentconfiguration-subcomponenttypeconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sub_component_type_configurations: Option<::ValueList<SubComponentTypeConfiguration>>,
    }

    impl ::codec::SerializeValue for ComponentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configuration_details) = self.configuration_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationDetails", configuration_details)?;
            }
            if let Some(ref sub_component_type_configurations) = self.sub_component_type_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubComponentTypeConfigurations", sub_component_type_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configuration_details: Option<::Value<ConfigurationDetails>> = None;
                    let mut sub_component_type_configurations: Option<::ValueList<SubComponentTypeConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigurationDetails" => {
                                configuration_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubComponentTypeConfigurations" => {
                                sub_component_type_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentConfiguration {
                        configuration_details: configuration_details,
                        sub_component_type_configurations: sub_component_type_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.ComponentMonitoringSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ComponentMonitoringSetting {
        /// Property [`ComponentARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html#cfn-applicationinsights-application-componentmonitoringsetting-componentarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_arn: Option<::Value<String>>,
        /// Property [`ComponentConfigurationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html#cfn-applicationinsights-application-componentmonitoringsetting-componentconfigurationmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_configuration_mode: ::Value<String>,
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html#cfn-applicationinsights-application-componentmonitoringsetting-componentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_name: Option<::Value<String>>,
        /// Property [`CustomComponentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html#cfn-applicationinsights-application-componentmonitoringsetting-customcomponentconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_component_configuration: Option<::Value<ComponentConfiguration>>,
        /// Property [`DefaultOverwriteComponentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html#cfn-applicationinsights-application-componentmonitoringsetting-defaultoverwritecomponentconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_overwrite_component_configuration: Option<::Value<ComponentConfiguration>>,
        /// Property [`Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html#cfn-applicationinsights-application-componentmonitoringsetting-tier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tier: ::Value<String>,
    }

    impl ::codec::SerializeValue for ComponentMonitoringSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_arn) = self.component_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentARN", component_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentConfigurationMode", &self.component_configuration_mode)?;
            if let Some(ref component_name) = self.component_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", component_name)?;
            }
            if let Some(ref custom_component_configuration) = self.custom_component_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomComponentConfiguration", custom_component_configuration)?;
            }
            if let Some(ref default_overwrite_component_configuration) = self.default_overwrite_component_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultOverwriteComponentConfiguration", default_overwrite_component_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tier", &self.tier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComponentMonitoringSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComponentMonitoringSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComponentMonitoringSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComponentMonitoringSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_arn: Option<::Value<String>> = None;
                    let mut component_configuration_mode: Option<::Value<String>> = None;
                    let mut component_name: Option<::Value<String>> = None;
                    let mut custom_component_configuration: Option<::Value<ComponentConfiguration>> = None;
                    let mut default_overwrite_component_configuration: Option<::Value<ComponentConfiguration>> = None;
                    let mut tier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentARN" => {
                                component_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentConfigurationMode" => {
                                component_configuration_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomComponentConfiguration" => {
                                custom_component_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultOverwriteComponentConfiguration" => {
                                default_overwrite_component_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tier" => {
                                tier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComponentMonitoringSetting {
                        component_arn: component_arn,
                        component_configuration_mode: component_configuration_mode.ok_or(::serde::de::Error::missing_field("ComponentConfigurationMode"))?,
                        component_name: component_name,
                        custom_component_configuration: custom_component_configuration,
                        default_overwrite_component_configuration: default_overwrite_component_configuration,
                        tier: tier.ok_or(::serde::de::Error::missing_field("Tier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.ConfigurationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigurationDetails {
        /// Property [`AlarmMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-alarmmetrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_metrics: Option<::ValueList<AlarmMetric>>,
        /// Property [`Alarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-alarms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarms: Option<::ValueList<Alarm>>,
        /// Property [`HAClusterPrometheusExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-haclusterprometheusexporter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ha_cluster_prometheus_exporter: Option<::Value<HAClusterPrometheusExporter>>,
        /// Property [`HANAPrometheusExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-hanaprometheusexporter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hana_prometheus_exporter: Option<::Value<HANAPrometheusExporter>>,
        /// Property [`JMXPrometheusExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-jmxprometheusexporter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jmx_prometheus_exporter: Option<::Value<JMXPrometheusExporter>>,
        /// Property [`Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-logs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logs: Option<::ValueList<Log>>,
        /// Property [`WindowsEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html#cfn-applicationinsights-application-configurationdetails-windowsevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub windows_events: Option<::ValueList<WindowsEvent>>,
    }

    impl ::codec::SerializeValue for ConfigurationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alarm_metrics) = self.alarm_metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmMetrics", alarm_metrics)?;
            }
            if let Some(ref alarms) = self.alarms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alarms", alarms)?;
            }
            if let Some(ref ha_cluster_prometheus_exporter) = self.ha_cluster_prometheus_exporter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HAClusterPrometheusExporter", ha_cluster_prometheus_exporter)?;
            }
            if let Some(ref hana_prometheus_exporter) = self.hana_prometheus_exporter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HANAPrometheusExporter", hana_prometheus_exporter)?;
            }
            if let Some(ref jmx_prometheus_exporter) = self.jmx_prometheus_exporter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JMXPrometheusExporter", jmx_prometheus_exporter)?;
            }
            if let Some(ref logs) = self.logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logs", logs)?;
            }
            if let Some(ref windows_events) = self.windows_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowsEvents", windows_events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_metrics: Option<::ValueList<AlarmMetric>> = None;
                    let mut alarms: Option<::ValueList<Alarm>> = None;
                    let mut ha_cluster_prometheus_exporter: Option<::Value<HAClusterPrometheusExporter>> = None;
                    let mut hana_prometheus_exporter: Option<::Value<HANAPrometheusExporter>> = None;
                    let mut jmx_prometheus_exporter: Option<::Value<JMXPrometheusExporter>> = None;
                    let mut logs: Option<::ValueList<Log>> = None;
                    let mut windows_events: Option<::ValueList<WindowsEvent>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmMetrics" => {
                                alarm_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Alarms" => {
                                alarms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HAClusterPrometheusExporter" => {
                                ha_cluster_prometheus_exporter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HANAPrometheusExporter" => {
                                hana_prometheus_exporter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JMXPrometheusExporter" => {
                                jmx_prometheus_exporter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logs" => {
                                logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WindowsEvents" => {
                                windows_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationDetails {
                        alarm_metrics: alarm_metrics,
                        alarms: alarms,
                        ha_cluster_prometheus_exporter: ha_cluster_prometheus_exporter,
                        hana_prometheus_exporter: hana_prometheus_exporter,
                        jmx_prometheus_exporter: jmx_prometheus_exporter,
                        logs: logs,
                        windows_events: windows_events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.CustomComponent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-customcomponent.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomComponent {
        /// Property [`ComponentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-customcomponent.html#cfn-applicationinsights-application-customcomponent-componentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_name: ::Value<String>,
        /// Property [`ResourceList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-customcomponent.html#cfn-applicationinsights-application-customcomponent-resourcelist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_list: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for CustomComponent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentName", &self.component_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceList", &self.resource_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomComponent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomComponent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomComponent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomComponent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_name: Option<::Value<String>> = None;
                    let mut resource_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentName" => {
                                component_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceList" => {
                                resource_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomComponent {
                        component_name: component_name.ok_or(::serde::de::Error::missing_field("ComponentName"))?,
                        resource_list: resource_list.ok_or(::serde::de::Error::missing_field("ResourceList"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.HAClusterPrometheusExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-haclusterprometheusexporter.html) property type.
    #[derive(Debug, Default)]
    pub struct HAClusterPrometheusExporter {
        /// Property [`PrometheusPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-haclusterprometheusexporter.html#cfn-applicationinsights-application-haclusterprometheusexporter-prometheusport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prometheus_port: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HAClusterPrometheusExporter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref prometheus_port) = self.prometheus_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrometheusPort", prometheus_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HAClusterPrometheusExporter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HAClusterPrometheusExporter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HAClusterPrometheusExporter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HAClusterPrometheusExporter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut prometheus_port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PrometheusPort" => {
                                prometheus_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HAClusterPrometheusExporter {
                        prometheus_port: prometheus_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.HANAPrometheusExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html) property type.
    #[derive(Debug, Default)]
    pub struct HANAPrometheusExporter {
        /// Property [`AgreeToInstallHANADBClient`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html#cfn-applicationinsights-application-hanaprometheusexporter-agreetoinstallhanadbclient).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub agree_to_install_hanadb_client: ::Value<bool>,
        /// Property [`HANAPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html#cfn-applicationinsights-application-hanaprometheusexporter-hanaport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hana_port: ::Value<String>,
        /// Property [`HANASID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html#cfn-applicationinsights-application-hanaprometheusexporter-hanasid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hanasid: ::Value<String>,
        /// Property [`HANASecretName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html#cfn-applicationinsights-application-hanaprometheusexporter-hanasecretname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hana_secret_name: ::Value<String>,
        /// Property [`PrometheusPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html#cfn-applicationinsights-application-hanaprometheusexporter-prometheusport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prometheus_port: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HANAPrometheusExporter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgreeToInstallHANADBClient", &self.agree_to_install_hanadb_client)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HANAPort", &self.hana_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HANASID", &self.hanasid)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HANASecretName", &self.hana_secret_name)?;
            if let Some(ref prometheus_port) = self.prometheus_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrometheusPort", prometheus_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HANAPrometheusExporter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HANAPrometheusExporter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HANAPrometheusExporter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HANAPrometheusExporter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut agree_to_install_hanadb_client: Option<::Value<bool>> = None;
                    let mut hana_port: Option<::Value<String>> = None;
                    let mut hanasid: Option<::Value<String>> = None;
                    let mut hana_secret_name: Option<::Value<String>> = None;
                    let mut prometheus_port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AgreeToInstallHANADBClient" => {
                                agree_to_install_hanadb_client = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HANAPort" => {
                                hana_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HANASID" => {
                                hanasid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HANASecretName" => {
                                hana_secret_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrometheusPort" => {
                                prometheus_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HANAPrometheusExporter {
                        agree_to_install_hanadb_client: agree_to_install_hanadb_client.ok_or(::serde::de::Error::missing_field("AgreeToInstallHANADBClient"))?,
                        hana_port: hana_port.ok_or(::serde::de::Error::missing_field("HANAPort"))?,
                        hanasid: hanasid.ok_or(::serde::de::Error::missing_field("HANASID"))?,
                        hana_secret_name: hana_secret_name.ok_or(::serde::de::Error::missing_field("HANASecretName"))?,
                        prometheus_port: prometheus_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.JMXPrometheusExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-jmxprometheusexporter.html) property type.
    #[derive(Debug, Default)]
    pub struct JMXPrometheusExporter {
        /// Property [`HostPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-jmxprometheusexporter.html#cfn-applicationinsights-application-jmxprometheusexporter-hostport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host_port: Option<::Value<String>>,
        /// Property [`JMXURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-jmxprometheusexporter.html#cfn-applicationinsights-application-jmxprometheusexporter-jmxurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jmxurl: Option<::Value<String>>,
        /// Property [`PrometheusPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-jmxprometheusexporter.html#cfn-applicationinsights-application-jmxprometheusexporter-prometheusport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prometheus_port: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JMXPrometheusExporter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref host_port) = self.host_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostPort", host_port)?;
            }
            if let Some(ref jmxurl) = self.jmxurl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JMXURL", jmxurl)?;
            }
            if let Some(ref prometheus_port) = self.prometheus_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrometheusPort", prometheus_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JMXPrometheusExporter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JMXPrometheusExporter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JMXPrometheusExporter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JMXPrometheusExporter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host_port: Option<::Value<String>> = None;
                    let mut jmxurl: Option<::Value<String>> = None;
                    let mut prometheus_port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HostPort" => {
                                host_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JMXURL" => {
                                jmxurl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrometheusPort" => {
                                prometheus_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JMXPrometheusExporter {
                        host_port: host_port,
                        jmxurl: jmxurl,
                        prometheus_port: prometheus_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.Log`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html) property type.
    #[derive(Debug, Default)]
    pub struct Log {
        /// Property [`Encoding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html#cfn-applicationinsights-application-log-encoding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encoding: Option<::Value<String>>,
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html#cfn-applicationinsights-application-log-loggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
        /// Property [`LogPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html#cfn-applicationinsights-application-log-logpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_path: Option<::Value<String>>,
        /// Property [`LogType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html#cfn-applicationinsights-application-log-logtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_type: ::Value<String>,
        /// Property [`PatternSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html#cfn-applicationinsights-application-log-patternset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_set: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Log {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encoding) = self.encoding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encoding", encoding)?;
            }
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            if let Some(ref log_path) = self.log_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPath", log_path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogType", &self.log_type)?;
            if let Some(ref pattern_set) = self.pattern_set {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternSet", pattern_set)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Log {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Log, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Log;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Log")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encoding: Option<::Value<String>> = None;
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut log_path: Option<::Value<String>> = None;
                    let mut log_type: Option<::Value<String>> = None;
                    let mut pattern_set: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encoding" => {
                                encoding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogPath" => {
                                log_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogType" => {
                                log_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternSet" => {
                                pattern_set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Log {
                        encoding: encoding,
                        log_group_name: log_group_name,
                        log_path: log_path,
                        log_type: log_type.ok_or(::serde::de::Error::missing_field("LogType"))?,
                        pattern_set: pattern_set,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.LogPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct LogPattern {
        /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpattern.html#cfn-applicationinsights-application-logpattern-pattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern: ::Value<String>,
        /// Property [`PatternName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpattern.html#cfn-applicationinsights-application-logpattern-patternname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_name: ::Value<String>,
        /// Property [`Rank`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpattern.html#cfn-applicationinsights-application-logpattern-rank).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rank: ::Value<u32>,
    }

    impl ::codec::SerializeValue for LogPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", &self.pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternName", &self.pattern_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rank", &self.rank)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pattern: Option<::Value<String>> = None;
                    let mut pattern_name: Option<::Value<String>> = None;
                    let mut rank: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pattern" => {
                                pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternName" => {
                                pattern_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rank" => {
                                rank = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogPattern {
                        pattern: pattern.ok_or(::serde::de::Error::missing_field("Pattern"))?,
                        pattern_name: pattern_name.ok_or(::serde::de::Error::missing_field("PatternName"))?,
                        rank: rank.ok_or(::serde::de::Error::missing_field("Rank"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.LogPatternSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpatternset.html) property type.
    #[derive(Debug, Default)]
    pub struct LogPatternSet {
        /// Property [`LogPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpatternset.html#cfn-applicationinsights-application-logpatternset-logpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_patterns: ::ValueList<LogPattern>,
        /// Property [`PatternSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpatternset.html#cfn-applicationinsights-application-logpatternset-patternsetname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_set_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for LogPatternSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPatterns", &self.log_patterns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternSetName", &self.pattern_set_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogPatternSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogPatternSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogPatternSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogPatternSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_patterns: Option<::ValueList<LogPattern>> = None;
                    let mut pattern_set_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogPatterns" => {
                                log_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternSetName" => {
                                pattern_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogPatternSet {
                        log_patterns: log_patterns.ok_or(::serde::de::Error::missing_field("LogPatterns"))?,
                        pattern_set_name: pattern_set_name.ok_or(::serde::de::Error::missing_field("PatternSetName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.SubComponentConfigurationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponentconfigurationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct SubComponentConfigurationDetails {
        /// Property [`AlarmMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponentconfigurationdetails.html#cfn-applicationinsights-application-subcomponentconfigurationdetails-alarmmetrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_metrics: Option<::ValueList<AlarmMetric>>,
        /// Property [`Logs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponentconfigurationdetails.html#cfn-applicationinsights-application-subcomponentconfigurationdetails-logs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logs: Option<::ValueList<Log>>,
        /// Property [`WindowsEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponentconfigurationdetails.html#cfn-applicationinsights-application-subcomponentconfigurationdetails-windowsevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub windows_events: Option<::ValueList<WindowsEvent>>,
    }

    impl ::codec::SerializeValue for SubComponentConfigurationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alarm_metrics) = self.alarm_metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmMetrics", alarm_metrics)?;
            }
            if let Some(ref logs) = self.logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logs", logs)?;
            }
            if let Some(ref windows_events) = self.windows_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowsEvents", windows_events)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubComponentConfigurationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubComponentConfigurationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubComponentConfigurationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubComponentConfigurationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_metrics: Option<::ValueList<AlarmMetric>> = None;
                    let mut logs: Option<::ValueList<Log>> = None;
                    let mut windows_events: Option<::ValueList<WindowsEvent>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmMetrics" => {
                                alarm_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logs" => {
                                logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WindowsEvents" => {
                                windows_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubComponentConfigurationDetails {
                        alarm_metrics: alarm_metrics,
                        logs: logs,
                        windows_events: windows_events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.SubComponentTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponenttypeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SubComponentTypeConfiguration {
        /// Property [`SubComponentConfigurationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponenttypeconfiguration.html#cfn-applicationinsights-application-subcomponenttypeconfiguration-subcomponentconfigurationdetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sub_component_configuration_details: ::Value<SubComponentConfigurationDetails>,
        /// Property [`SubComponentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponenttypeconfiguration.html#cfn-applicationinsights-application-subcomponenttypeconfiguration-subcomponenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sub_component_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SubComponentTypeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubComponentConfigurationDetails", &self.sub_component_configuration_details)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubComponentType", &self.sub_component_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubComponentTypeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubComponentTypeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubComponentTypeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubComponentTypeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sub_component_configuration_details: Option<::Value<SubComponentConfigurationDetails>> = None;
                    let mut sub_component_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubComponentConfigurationDetails" => {
                                sub_component_configuration_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubComponentType" => {
                                sub_component_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubComponentTypeConfiguration {
                        sub_component_configuration_details: sub_component_configuration_details.ok_or(::serde::de::Error::missing_field("SubComponentConfigurationDetails"))?,
                        sub_component_type: sub_component_type.ok_or(::serde::de::Error::missing_field("SubComponentType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationInsights::Application.WindowsEvent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-windowsevent.html) property type.
    #[derive(Debug, Default)]
    pub struct WindowsEvent {
        /// Property [`EventLevels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-windowsevent.html#cfn-applicationinsights-application-windowsevent-eventlevels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_levels: ::ValueList<String>,
        /// Property [`EventName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-windowsevent.html#cfn-applicationinsights-application-windowsevent-eventname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_name: ::Value<String>,
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-windowsevent.html#cfn-applicationinsights-application-windowsevent-loggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_name: ::Value<String>,
        /// Property [`PatternSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-windowsevent.html#cfn-applicationinsights-application-windowsevent-patternset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_set: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WindowsEvent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventLevels", &self.event_levels)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventName", &self.event_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
            if let Some(ref pattern_set) = self.pattern_set {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternSet", pattern_set)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WindowsEvent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WindowsEvent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WindowsEvent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WindowsEvent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_levels: Option<::ValueList<String>> = None;
                    let mut event_name: Option<::Value<String>> = None;
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut pattern_set: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventLevels" => {
                                event_levels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventName" => {
                                event_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternSet" => {
                                pattern_set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WindowsEvent {
                        event_levels: event_levels.ok_or(::serde::de::Error::missing_field("EventLevels"))?,
                        event_name: event_name.ok_or(::serde::de::Error::missing_field("EventName"))?,
                        log_group_name: log_group_name.ok_or(::serde::de::Error::missing_field("LogGroupName"))?,
                        pattern_set: pattern_set,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
