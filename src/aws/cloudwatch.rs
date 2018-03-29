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
    #[serde(rename="ActionsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,
    /// Property `AlarmActions`.
    #[serde(rename="AlarmActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,
    /// Property `AlarmDescription`.
    #[serde(rename="AlarmDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,
    /// Property `AlarmName`.
    #[serde(rename="AlarmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    /// Property `ComparisonOperator`.
    #[serde(rename="ComparisonOperator")]
    pub comparison_operator: String,
    /// Property `Dimensions`.
    #[serde(rename="Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<self::alarm::Dimension>>,
    /// Property `EvaluateLowSampleCountPercentile`.
    #[serde(rename="EvaluateLowSampleCountPercentile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_low_sample_count_percentile: Option<String>,
    /// Property `EvaluationPeriods`.
    #[serde(rename="EvaluationPeriods")]
    pub evaluation_periods: u32,
    /// Property `ExtendedStatistic`.
    #[serde(rename="ExtendedStatistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<String>,
    /// Property `InsufficientDataActions`.
    #[serde(rename="InsufficientDataActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,
    /// Property `MetricName`.
    #[serde(rename="MetricName")]
    pub metric_name: String,
    /// Property `Namespace`.
    #[serde(rename="Namespace")]
    pub namespace: String,
    /// Property `OKActions`.
    #[serde(rename="OKActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_actions: Option<Vec<String>>,
    /// Property `Period`.
    #[serde(rename="Period")]
    pub period: u32,
    /// Property `Statistic`.
    #[serde(rename="Statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    /// Property `Threshold`.
    #[serde(rename="Threshold")]
    pub threshold: f64,
    /// Property `TreatMissingData`.
    #[serde(rename="TreatMissingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
    /// Property `Unit`.
    #[serde(rename="Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
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
    #[serde(rename="DashboardBody")]
    pub dashboard_body: String,
    /// Property `DashboardName`.
    #[serde(rename="DashboardName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
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
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}
