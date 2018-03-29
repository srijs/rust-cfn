/// The [`AWS::ApplicationAutoScaling::ScalableTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ScalableTarget {
    properties: ScalableTargetProperties
}

/// Properties for the `ScalableTarget` resource.
#[derive(Serialize, Deserialize)]
pub struct ScalableTargetProperties {
    #[serde(rename="MaxCapacity")]
    pub max_capacity: u32,
    #[serde(rename="MinCapacity")]
    pub min_capacity: u32,
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[serde(rename="ScheduledActions")]
    pub scheduled_actions: Vec<self::scalable_target::ScheduledAction>,
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

impl<'a> ::Resource<'a> for ScalableTarget {
    type Properties = ScalableTargetProperties;
    const TYPE: &'static str = "AWS::ApplicationAutoScaling::ScalableTarget";
    fn properties(&self) -> &ScalableTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScalableTargetProperties {
        &mut self.properties
    }
}

impl From<ScalableTargetProperties> for ScalableTarget {
    fn from(properties: ScalableTargetProperties) -> ScalableTarget {
        ScalableTarget { properties }
    }
}

/// The [`AWS::ApplicationAutoScaling::ScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct ScalingPolicyProperties {
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    #[serde(rename="PolicyType")]
    pub policy_type: String,
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[serde(rename="ScalingTargetId")]
    pub scaling_target_id: String,
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
    #[serde(rename="StepScalingPolicyConfiguration")]
    pub step_scaling_policy_configuration: self::scaling_policy::StepScalingPolicyConfiguration,
    #[serde(rename="TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: self::scaling_policy::TargetTrackingScalingPolicyConfiguration,
}

impl<'a> ::Resource<'a> for ScalingPolicy {
    type Properties = ScalingPolicyProperties;
    const TYPE: &'static str = "AWS::ApplicationAutoScaling::ScalingPolicy";
    fn properties(&self) -> &ScalingPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScalingPolicyProperties {
        &mut self.properties
    }
}

impl From<ScalingPolicyProperties> for ScalingPolicy {
    fn from(properties: ScalingPolicyProperties) -> ScalingPolicy {
        ScalingPolicy { properties }
    }
}

pub mod scalable_target {
    #[derive(Serialize, Deserialize)]
    pub struct ScalableTargetAction {
        #[serde(rename="MaxCapacity")]
        pub max_capacity: u32,
        #[serde(rename="MinCapacity")]
        pub min_capacity: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScheduledAction {
        #[serde(rename="EndTime")]
        pub end_time: String,
        #[serde(rename="ScalableTargetAction")]
        pub scalable_target_action: ScalableTargetAction,
        #[serde(rename="Schedule")]
        pub schedule: String,
        #[serde(rename="ScheduledActionName")]
        pub scheduled_action_name: String,
        #[serde(rename="StartTime")]
        pub start_time: String,
    }

}

pub mod scaling_policy {
    #[derive(Serialize, Deserialize)]
    pub struct CustomizedMetricSpecification {
        #[serde(rename="Dimensions")]
        pub dimensions: Vec<MetricDimension>,
        #[serde(rename="MetricName")]
        pub metric_name: String,
        #[serde(rename="Namespace")]
        pub namespace: String,
        #[serde(rename="Statistic")]
        pub statistic: String,
        #[serde(rename="Unit")]
        pub unit: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetricDimension {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PredefinedMetricSpecification {
        #[serde(rename="PredefinedMetricType")]
        pub predefined_metric_type: String,
        #[serde(rename="ResourceLabel")]
        pub resource_label: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StepAdjustment {
        #[serde(rename="MetricIntervalLowerBound")]
        pub metric_interval_lower_bound: f64,
        #[serde(rename="MetricIntervalUpperBound")]
        pub metric_interval_upper_bound: f64,
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StepScalingPolicyConfiguration {
        #[serde(rename="AdjustmentType")]
        pub adjustment_type: String,
        #[serde(rename="Cooldown")]
        pub cooldown: u32,
        #[serde(rename="MetricAggregationType")]
        pub metric_aggregation_type: String,
        #[serde(rename="MinAdjustmentMagnitude")]
        pub min_adjustment_magnitude: u32,
        #[serde(rename="StepAdjustments")]
        pub step_adjustments: Vec<StepAdjustment>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        #[serde(rename="CustomizedMetricSpecification")]
        pub customized_metric_specification: CustomizedMetricSpecification,
        #[serde(rename="DisableScaleIn")]
        pub disable_scale_in: bool,
        #[serde(rename="PredefinedMetricSpecification")]
        pub predefined_metric_specification: PredefinedMetricSpecification,
        #[serde(rename="ScaleInCooldown")]
        pub scale_in_cooldown: u32,
        #[serde(rename="ScaleOutCooldown")]
        pub scale_out_cooldown: u32,
        #[serde(rename="TargetValue")]
        pub target_value: f64,
    }

}

