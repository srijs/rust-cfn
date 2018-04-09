//! Types for the `CloudWatch` service.

/// The [`AWS::CloudWatch::Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html) resource type.
#[derive(Debug)]
pub struct Alarm {
    properties: AlarmProperties
}

/// Properties for the `Alarm` resource.
#[derive(Debug)]
pub struct AlarmProperties {
    /// Property `ActionsEnabled`.
    pub actions_enabled: Option<::Value<bool>>,
    /// Property `AlarmActions`.
    pub alarm_actions: Option<::ValueList<String>>,
    /// Property `AlarmDescription`.
    pub alarm_description: Option<::Value<String>>,
    /// Property `AlarmName`.
    pub alarm_name: Option<::Value<String>>,
    /// Property `ComparisonOperator`.
    pub comparison_operator: ::Value<String>,
    /// Property `Dimensions`.
    pub dimensions: Option<::ValueList<self::alarm::Dimension>>,
    /// Property `EvaluateLowSampleCountPercentile`.
    pub evaluate_low_sample_count_percentile: Option<::Value<String>>,
    /// Property `EvaluationPeriods`.
    pub evaluation_periods: ::Value<u32>,
    /// Property `ExtendedStatistic`.
    pub extended_statistic: Option<::Value<String>>,
    /// Property `InsufficientDataActions`.
    pub insufficient_data_actions: Option<::ValueList<String>>,
    /// Property `MetricName`.
    pub metric_name: ::Value<String>,
    /// Property `Namespace`.
    pub namespace: ::Value<String>,
    /// Property `OKActions`.
    pub ok_actions: Option<::ValueList<String>>,
    /// Property `Period`.
    pub period: ::Value<u32>,
    /// Property `Statistic`.
    pub statistic: Option<::Value<String>>,
    /// Property `Threshold`.
    pub threshold: ::Value<f64>,
    /// Property `TreatMissingData`.
    pub treat_missing_data: Option<::Value<String>>,
    /// Property `Unit`.
    pub unit: Option<::Value<String>>,
}

impl ::serde::Serialize for AlarmProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionsEnabled", &self.actions_enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmActions", &self.alarm_actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmDescription", &self.alarm_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluateLowSampleCountPercentile", &self.evaluate_low_sample_count_percentile)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", &self.evaluation_periods)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedStatistic", &self.extended_statistic)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsufficientDataActions", &self.insufficient_data_actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OKActions", &self.ok_actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatMissingData", &self.treat_missing_data)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AlarmProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AlarmProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AlarmProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions_enabled = None;
                let mut alarm_actions = None;
                let mut alarm_description = None;
                let mut alarm_name = None;
                let mut comparison_operator = None;
                let mut dimensions = None;
                let mut evaluate_low_sample_count_percentile = None;
                let mut evaluation_periods = None;
                let mut extended_statistic = None;
                let mut insufficient_data_actions = None;
                let mut metric_name = None;
                let mut namespace = None;
                let mut ok_actions = None;
                let mut period = None;
                let mut statistic = None;
                let mut threshold = None;
                let mut treat_missing_data = None;
                let mut unit = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionsEnabled" => {
                            actions_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AlarmActions" => {
                            alarm_actions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AlarmDescription" => {
                            alarm_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AlarmName" => {
                            alarm_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ComparisonOperator" => {
                            comparison_operator = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Dimensions" => {
                            dimensions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EvaluateLowSampleCountPercentile" => {
                            evaluate_low_sample_count_percentile = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EvaluationPeriods" => {
                            evaluation_periods = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ExtendedStatistic" => {
                            extended_statistic = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InsufficientDataActions" => {
                            insufficient_data_actions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MetricName" => {
                            metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Namespace" => {
                            namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OKActions" => {
                            ok_actions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Period" => {
                            period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Statistic" => {
                            statistic = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Threshold" => {
                            threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TreatMissingData" => {
                            treat_missing_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Unit" => {
                            unit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AlarmProperties {
                    actions_enabled: actions_enabled,
                    alarm_actions: alarm_actions,
                    alarm_description: alarm_description,
                    alarm_name: alarm_name,
                    comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                    dimensions: dimensions,
                    evaluate_low_sample_count_percentile: evaluate_low_sample_count_percentile,
                    evaluation_periods: evaluation_periods.ok_or(::serde::de::Error::missing_field("EvaluationPeriods"))?,
                    extended_statistic: extended_statistic,
                    insufficient_data_actions: insufficient_data_actions,
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    ok_actions: ok_actions,
                    period: period.ok_or(::serde::de::Error::missing_field("Period"))?,
                    statistic: statistic,
                    threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                    treat_missing_data: treat_missing_data,
                    unit: unit,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Alarm {
    type Properties = AlarmProperties;
    const TYPE: &'static str = "AWS::CloudWatch::Alarm";
    fn properties(&self) -> &AlarmProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AlarmProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alarm {}

impl From<AlarmProperties> for Alarm {
    fn from(properties: AlarmProperties) -> Alarm {
        Alarm { properties }
    }
}

/// The [`AWS::CloudWatch::Dashboard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html) resource type.
#[derive(Debug)]
pub struct Dashboard {
    properties: DashboardProperties
}

/// Properties for the `Dashboard` resource.
#[derive(Debug)]
pub struct DashboardProperties {
    /// Property `DashboardBody`.
    pub dashboard_body: ::Value<String>,
    /// Property `DashboardName`.
    pub dashboard_name: Option<::Value<String>>,
}

impl ::serde::Serialize for DashboardProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardBody", &self.dashboard_body)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardName", &self.dashboard_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DashboardProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DashboardProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DashboardProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dashboard_body = None;
                let mut dashboard_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DashboardBody" => {
                            dashboard_body = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DashboardName" => {
                            dashboard_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DashboardProperties {
                    dashboard_body: dashboard_body.ok_or(::serde::de::Error::missing_field("DashboardBody"))?,
                    dashboard_name: dashboard_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Dashboard {
    type Properties = DashboardProperties;
    const TYPE: &'static str = "AWS::CloudWatch::Dashboard";
    fn properties(&self) -> &DashboardProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DashboardProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dashboard {}

impl From<DashboardProperties> for Dashboard {
    fn from(properties: DashboardProperties) -> Dashboard {
        Dashboard { properties }
    }
}

pub mod alarm {
    //! Property types for the `Alarm` resource.

    /// The [`AWS::CloudWatch::Alarm.Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-dimension.html) property type.
    #[derive(Debug)]
    pub struct Dimension {
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Dimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Dimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Dimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Dimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Dimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Dimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
