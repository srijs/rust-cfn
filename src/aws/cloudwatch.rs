//! Types for the `CloudWatch` service.

/// The [`AWS::CloudWatch::Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html) resource type.
#[derive(Debug)]
pub struct Alarm {
    properties: AlarmProperties
}

/// Properties for the `Alarm` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmProperties {
    /// Property `ActionsEnabled`.
    #[serde(rename = "ActionsEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<::Value<bool>>,
    /// Property `AlarmActions`.
    #[serde(rename = "AlarmActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<::ValueList<String>>,
    /// Property `AlarmDescription`.
    #[serde(rename = "AlarmDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<::Value<String>>,
    /// Property `AlarmName`.
    #[serde(rename = "AlarmName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<::Value<String>>,
    /// Property `ComparisonOperator`.
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: ::Value<String>,
    /// Property `Dimensions`.
    #[serde(rename = "Dimensions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<::ValueList<self::alarm::Dimension>>,
    /// Property `EvaluateLowSampleCountPercentile`.
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluate_low_sample_count_percentile: Option<::Value<String>>,
    /// Property `EvaluationPeriods`.
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: ::Value<u32>,
    /// Property `ExtendedStatistic`.
    #[serde(rename = "ExtendedStatistic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<::Value<String>>,
    /// Property `InsufficientDataActions`.
    #[serde(rename = "InsufficientDataActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<::ValueList<String>>,
    /// Property `MetricName`.
    #[serde(rename = "MetricName")]
    pub metric_name: ::Value<String>,
    /// Property `Namespace`.
    #[serde(rename = "Namespace")]
    pub namespace: ::Value<String>,
    /// Property `OKActions`.
    #[serde(rename = "OKActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ok_actions: Option<::ValueList<String>>,
    /// Property `Period`.
    #[serde(rename = "Period")]
    pub period: ::Value<u32>,
    /// Property `Statistic`.
    #[serde(rename = "Statistic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistic: Option<::Value<String>>,
    /// Property `Threshold`.
    #[serde(rename = "Threshold")]
    pub threshold: ::Value<f64>,
    /// Property `TreatMissingData`.
    #[serde(rename = "TreatMissingData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<::Value<String>>,
    /// Property `Unit`.
    #[serde(rename = "Unit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<::Value<String>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardProperties {
    /// Property `DashboardBody`.
    #[serde(rename = "DashboardBody")]
    pub dashboard_body: ::Value<String>,
    /// Property `DashboardName`.
    #[serde(rename = "DashboardName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<::Value<String>>,
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Dimension {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Dimension);
}
