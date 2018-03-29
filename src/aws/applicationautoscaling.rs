//! Types for the `ApplicationAutoScaling` service.

/// The [`AWS::ApplicationAutoScaling::ScalableTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html) resource type.
#[derive(Debug)]
pub struct ScalableTarget {
    properties: ScalableTargetProperties
}

/// Properties for the `ScalableTarget` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScalableTargetProperties {
    /// Property `MaxCapacity`.
    #[serde(rename="MaxCapacity")]
    pub max_capacity: u32,
    /// Property `MinCapacity`.
    #[serde(rename="MinCapacity")]
    pub min_capacity: u32,
    /// Property `ResourceId`.
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    /// Property `RoleARN`.
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    /// Property `ScalableDimension`.
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    /// Property `ScheduledActions`.
    #[serde(rename="ScheduledActions")]
    pub scheduled_actions: Vec<self::scalable_target::ScheduledAction>,
    /// Property `ServiceNamespace`.
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

impl ::private::Sealed for ScalableTarget {}

impl From<ScalableTargetProperties> for ScalableTarget {
    fn from(properties: ScalableTargetProperties) -> ScalableTarget {
        ScalableTarget { properties }
    }
}

/// The [`AWS::ApplicationAutoScaling::ScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html) resource type.
#[derive(Debug)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingPolicyProperties {
    /// Property `PolicyName`.
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    /// Property `PolicyType`.
    #[serde(rename="PolicyType")]
    pub policy_type: String,
    /// Property `ResourceId`.
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    /// Property `ScalableDimension`.
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    /// Property `ScalingTargetId`.
    #[serde(rename="ScalingTargetId")]
    pub scaling_target_id: String,
    /// Property `ServiceNamespace`.
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
    /// Property `StepScalingPolicyConfiguration`.
    #[serde(rename="StepScalingPolicyConfiguration")]
    pub step_scaling_policy_configuration: self::scaling_policy::StepScalingPolicyConfiguration,
    /// Property `TargetTrackingScalingPolicyConfiguration`.
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

impl ::private::Sealed for ScalingPolicy {}

impl From<ScalingPolicyProperties> for ScalingPolicy {
    fn from(properties: ScalingPolicyProperties) -> ScalingPolicy {
        ScalingPolicy { properties }
    }
}

pub mod scalable_target {
    //! Property types for the `ScalableTarget` resource.

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.ScalableTargetAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scalabletargetaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalableTargetAction {
        /// Property `MaxCapacity`.
        #[serde(rename="MaxCapacity")]
        pub max_capacity: u32,
        /// Property `MinCapacity`.
        #[serde(rename="MinCapacity")]
        pub min_capacity: u32,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScheduledAction {
        /// Property `EndTime`.
        #[serde(rename="EndTime")]
        pub end_time: String,
        /// Property `ScalableTargetAction`.
        #[serde(rename="ScalableTargetAction")]
        pub scalable_target_action: ScalableTargetAction,
        /// Property `Schedule`.
        #[serde(rename="Schedule")]
        pub schedule: String,
        /// Property `ScheduledActionName`.
        #[serde(rename="ScheduledActionName")]
        pub scheduled_action_name: String,
        /// Property `StartTime`.
        #[serde(rename="StartTime")]
        pub start_time: String,
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomizedMetricSpecification {
        /// Property `Dimensions`.
        #[serde(rename="Dimensions")]
        pub dimensions: Vec<MetricDimension>,
        /// Property `MetricName`.
        #[serde(rename="MetricName")]
        pub metric_name: String,
        /// Property `Namespace`.
        #[serde(rename="Namespace")]
        pub namespace: String,
        /// Property `Statistic`.
        #[serde(rename="Statistic")]
        pub statistic: String,
        /// Property `Unit`.
        #[serde(rename="Unit")]
        pub unit: String,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PredefinedMetricSpecification {
        /// Property `PredefinedMetricType`.
        #[serde(rename="PredefinedMetricType")]
        pub predefined_metric_type: String,
        /// Property `ResourceLabel`.
        #[serde(rename="ResourceLabel")]
        pub resource_label: String,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepAdjustment {
        /// Property `MetricIntervalLowerBound`.
        #[serde(rename="MetricIntervalLowerBound")]
        pub metric_interval_lower_bound: f64,
        /// Property `MetricIntervalUpperBound`.
        #[serde(rename="MetricIntervalUpperBound")]
        pub metric_interval_upper_bound: f64,
        /// Property `ScalingAdjustment`.
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename="AdjustmentType")]
        pub adjustment_type: String,
        /// Property `Cooldown`.
        #[serde(rename="Cooldown")]
        pub cooldown: u32,
        /// Property `MetricAggregationType`.
        #[serde(rename="MetricAggregationType")]
        pub metric_aggregation_type: String,
        /// Property `MinAdjustmentMagnitude`.
        #[serde(rename="MinAdjustmentMagnitude")]
        pub min_adjustment_magnitude: u32,
        /// Property `StepAdjustments`.
        #[serde(rename="StepAdjustments")]
        pub step_adjustments: Vec<StepAdjustment>,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        /// Property `CustomizedMetricSpecification`.
        #[serde(rename="CustomizedMetricSpecification")]
        pub customized_metric_specification: CustomizedMetricSpecification,
        /// Property `DisableScaleIn`.
        #[serde(rename="DisableScaleIn")]
        pub disable_scale_in: bool,
        /// Property `PredefinedMetricSpecification`.
        #[serde(rename="PredefinedMetricSpecification")]
        pub predefined_metric_specification: PredefinedMetricSpecification,
        /// Property `ScaleInCooldown`.
        #[serde(rename="ScaleInCooldown")]
        pub scale_in_cooldown: u32,
        /// Property `ScaleOutCooldown`.
        #[serde(rename="ScaleOutCooldown")]
        pub scale_out_cooldown: u32,
        /// Property `TargetValue`.
        #[serde(rename="TargetValue")]
        pub target_value: f64,
    }
}
