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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<Vec<self::scalable_target::ScheduledAction>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Property `ScalableDimension`.
    #[serde(rename="ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// Property `ScalingTargetId`.
    #[serde(rename="ScalingTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_target_id: Option<String>,
    /// Property `ServiceNamespace`.
    #[serde(rename="ServiceNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    /// Property `StepScalingPolicyConfiguration`.
    #[serde(rename="StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<self::scaling_policy::StepScalingPolicyConfiguration>,
    /// Property `TargetTrackingScalingPolicyConfiguration`.
    #[serde(rename="TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration: Option<self::scaling_policy::TargetTrackingScalingPolicyConfiguration>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_capacity: Option<u32>,
        /// Property `MinCapacity`.
        #[serde(rename="MinCapacity")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_capacity: Option<u32>,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScheduledAction {
        /// Property `EndTime`.
        #[serde(rename="EndTime")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<String>,
        /// Property `ScalableTargetAction`.
        #[serde(rename="ScalableTargetAction")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scalable_target_action: Option<ScalableTargetAction>,
        /// Property `Schedule`.
        #[serde(rename="Schedule")]
        pub schedule: String,
        /// Property `ScheduledActionName`.
        #[serde(rename="ScheduledActionName")]
        pub scheduled_action_name: String,
        /// Property `StartTime`.
        #[serde(rename="StartTime")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<String>,
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomizedMetricSpecification {
        /// Property `Dimensions`.
        #[serde(rename="Dimensions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dimensions: Option<Vec<MetricDimension>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_label: Option<String>,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepAdjustment {
        /// Property `MetricIntervalLowerBound`.
        #[serde(rename="MetricIntervalLowerBound")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metric_interval_lower_bound: Option<f64>,
        /// Property `MetricIntervalUpperBound`.
        #[serde(rename="MetricIntervalUpperBound")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metric_interval_upper_bound: Option<f64>,
        /// Property `ScalingAdjustment`.
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename="AdjustmentType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub adjustment_type: Option<String>,
        /// Property `Cooldown`.
        #[serde(rename="Cooldown")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cooldown: Option<u32>,
        /// Property `MetricAggregationType`.
        #[serde(rename="MetricAggregationType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metric_aggregation_type: Option<String>,
        /// Property `MinAdjustmentMagnitude`.
        #[serde(rename="MinAdjustmentMagnitude")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_adjustment_magnitude: Option<u32>,
        /// Property `StepAdjustments`.
        #[serde(rename="StepAdjustments")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub step_adjustments: Option<Vec<StepAdjustment>>,
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        /// Property `CustomizedMetricSpecification`.
        #[serde(rename="CustomizedMetricSpecification")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customized_metric_specification: Option<CustomizedMetricSpecification>,
        /// Property `DisableScaleIn`.
        #[serde(rename="DisableScaleIn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disable_scale_in: Option<bool>,
        /// Property `PredefinedMetricSpecification`.
        #[serde(rename="PredefinedMetricSpecification")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
        /// Property `ScaleInCooldown`.
        #[serde(rename="ScaleInCooldown")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scale_in_cooldown: Option<u32>,
        /// Property `ScaleOutCooldown`.
        #[serde(rename="ScaleOutCooldown")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scale_out_cooldown: Option<u32>,
        /// Property `TargetValue`.
        #[serde(rename="TargetValue")]
        pub target_value: f64,
    }
}
