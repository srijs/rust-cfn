/// The [`AWS::CloudWatch::Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Alarm {
    properties: AlarmProperties
}

/// Properties for the `Alarm` resource.
#[derive(Serialize, Deserialize)]
pub struct AlarmProperties {
    #[serde(rename="ActionsEnabled")]
    pub actions_enabled: (),
    #[serde(rename="AlarmActions")]
    pub alarm_actions: (),
    #[serde(rename="AlarmDescription")]
    pub alarm_description: (),
    #[serde(rename="AlarmName")]
    pub alarm_name: (),
    #[serde(rename="ComparisonOperator")]
    pub comparison_operator: (),
    #[serde(rename="Dimensions")]
    pub dimensions: (),
    #[serde(rename="EvaluateLowSampleCountPercentile")]
    pub evaluate_low_sample_count_percentile: (),
    #[serde(rename="EvaluationPeriods")]
    pub evaluation_periods: (),
    #[serde(rename="ExtendedStatistic")]
    pub extended_statistic: (),
    #[serde(rename="InsufficientDataActions")]
    pub insufficient_data_actions: (),
    #[serde(rename="MetricName")]
    pub metric_name: (),
    #[serde(rename="Namespace")]
    pub namespace: (),
    #[serde(rename="OKActions")]
    pub ok_actions: (),
    #[serde(rename="Period")]
    pub period: (),
    #[serde(rename="Statistic")]
    pub statistic: (),
    #[serde(rename="Threshold")]
    pub threshold: (),
    #[serde(rename="TreatMissingData")]
    pub treat_missing_data: (),
    #[serde(rename="Unit")]
    pub unit: (),
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

impl From<AlarmProperties> for Alarm {
    fn from(properties: AlarmProperties) -> Alarm {
        Alarm { properties }
    }
}

/// The [`AWS::CloudWatch::Dashboard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Dashboard {
    properties: DashboardProperties
}

/// Properties for the `Dashboard` resource.
#[derive(Serialize, Deserialize)]
pub struct DashboardProperties {
    #[serde(rename="DashboardBody")]
    pub dashboard_body: (),
    #[serde(rename="DashboardName")]
    pub dashboard_name: (),
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

impl From<DashboardProperties> for Dashboard {
    fn from(properties: DashboardProperties) -> Dashboard {
        Dashboard { properties }
    }
}

