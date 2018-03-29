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
    pub actions_enabled: bool,
    /// Property `AlarmActions`.
    #[serde(rename="AlarmActions")]
    pub alarm_actions: Vec<String>,
    /// Property `AlarmDescription`.
    #[serde(rename="AlarmDescription")]
    pub alarm_description: String,
    /// Property `AlarmName`.
    #[serde(rename="AlarmName")]
    pub alarm_name: String,
    /// Property `ComparisonOperator`.
    #[serde(rename="ComparisonOperator")]
    pub comparison_operator: String,
    /// Property `Dimensions`.
    #[serde(rename="Dimensions")]
    pub dimensions: Vec<self::alarm::Dimension>,
    /// Property `EvaluateLowSampleCountPercentile`.
    #[serde(rename="EvaluateLowSampleCountPercentile")]
    pub evaluate_low_sample_count_percentile: String,
    /// Property `EvaluationPeriods`.
    #[serde(rename="EvaluationPeriods")]
    pub evaluation_periods: u32,
    /// Property `ExtendedStatistic`.
    #[serde(rename="ExtendedStatistic")]
    pub extended_statistic: String,
    /// Property `InsufficientDataActions`.
    #[serde(rename="InsufficientDataActions")]
    pub insufficient_data_actions: Vec<String>,
    /// Property `MetricName`.
    #[serde(rename="MetricName")]
    pub metric_name: String,
    /// Property `Namespace`.
    #[serde(rename="Namespace")]
    pub namespace: String,
    /// Property `OKActions`.
    #[serde(rename="OKActions")]
    pub ok_actions: Vec<String>,
    /// Property `Period`.
    #[serde(rename="Period")]
    pub period: u32,
    /// Property `Statistic`.
    #[serde(rename="Statistic")]
    pub statistic: String,
    /// Property `Threshold`.
    #[serde(rename="Threshold")]
    pub threshold: f64,
    /// Property `TreatMissingData`.
    #[serde(rename="TreatMissingData")]
    pub treat_missing_data: String,
    /// Property `Unit`.
    #[serde(rename="Unit")]
    pub unit: String,
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
    pub dashboard_name: String,
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
