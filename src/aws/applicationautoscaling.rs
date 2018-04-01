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
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: ::Value<u32>,
    /// Property `MinCapacity`.
    #[serde(rename = "MinCapacity")]
    pub min_capacity: ::Value<u32>,
    /// Property `ResourceId`.
    #[serde(rename = "ResourceId")]
    pub resource_id: ::Value<String>,
    /// Property `RoleARN`.
    #[serde(rename = "RoleARN")]
    pub role_arn: ::Value<String>,
    /// Property `ScalableDimension`.
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: ::Value<String>,
    /// Property `ScheduledActions`.
    #[serde(rename = "ScheduledActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<::ValueList<self::scalable_target::ScheduledAction>>,
    /// Property `ServiceNamespace`.
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: ::Value<String>,
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
    #[serde(rename = "PolicyName")]
    pub policy_name: ::Value<String>,
    /// Property `PolicyType`.
    #[serde(rename = "PolicyType")]
    pub policy_type: ::Value<String>,
    /// Property `ResourceId`.
    #[serde(rename = "ResourceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<::Value<String>>,
    /// Property `ScalableDimension`.
    #[serde(rename = "ScalableDimension")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<::Value<String>>,
    /// Property `ScalingTargetId`.
    #[serde(rename = "ScalingTargetId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaling_target_id: Option<::Value<String>>,
    /// Property `ServiceNamespace`.
    #[serde(rename = "ServiceNamespace")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<::Value<String>>,
    /// Property `StepScalingPolicyConfiguration`.
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<::Value<self::scaling_policy::StepScalingPolicyConfiguration>>,
    /// Property `TargetTrackingScalingPolicyConfiguration`.
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration: Option<::Value<self::scaling_policy::TargetTrackingScalingPolicyConfiguration>>,
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
        #[serde(rename = "MaxCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_capacity: Option<::Value<u32>>,
        /// Property `MinCapacity`.
        #[serde(rename = "MinCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_capacity: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(ScalableTargetAction);

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScheduledAction {
        /// Property `EndTime`.
        #[serde(rename = "EndTime")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end_time: Option<::Value<String>>,
        /// Property `ScalableTargetAction`.
        #[serde(rename = "ScalableTargetAction")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scalable_target_action: Option<::Value<ScalableTargetAction>>,
        /// Property `Schedule`.
        #[serde(rename = "Schedule")]
        pub schedule: ::Value<String>,
        /// Property `ScheduledActionName`.
        #[serde(rename = "ScheduledActionName")]
        pub scheduled_action_name: ::Value<String>,
        /// Property `StartTime`.
        #[serde(rename = "StartTime")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start_time: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ScheduledAction);
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomizedMetricSpecification {
        /// Property `Dimensions`.
        #[serde(rename = "Dimensions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `MetricName`.
        #[serde(rename = "MetricName")]
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        #[serde(rename = "Namespace")]
        pub namespace: ::Value<String>,
        /// Property `Statistic`.
        #[serde(rename = "Statistic")]
        pub statistic: ::Value<String>,
        /// Property `Unit`.
        #[serde(rename = "Unit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CustomizedMetricSpecification);

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(MetricDimension);

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PredefinedMetricSpecification {
        /// Property `PredefinedMetricType`.
        #[serde(rename = "PredefinedMetricType")]
        pub predefined_metric_type: ::Value<String>,
        /// Property `ResourceLabel`.
        #[serde(rename = "ResourceLabel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_label: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(PredefinedMetricSpecification);

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepAdjustment {
        /// Property `MetricIntervalLowerBound`.
        #[serde(rename = "MetricIntervalLowerBound")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metric_interval_lower_bound: Option<::Value<f64>>,
        /// Property `MetricIntervalUpperBound`.
        #[serde(rename = "MetricIntervalUpperBound")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metric_interval_upper_bound: Option<::Value<f64>>,
        /// Property `ScalingAdjustment`.
        #[serde(rename = "ScalingAdjustment")]
        pub scaling_adjustment: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(StepAdjustment);

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename = "AdjustmentType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub adjustment_type: Option<::Value<String>>,
        /// Property `Cooldown`.
        #[serde(rename = "Cooldown")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cooldown: Option<::Value<u32>>,
        /// Property `MetricAggregationType`.
        #[serde(rename = "MetricAggregationType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metric_aggregation_type: Option<::Value<String>>,
        /// Property `MinAdjustmentMagnitude`.
        #[serde(rename = "MinAdjustmentMagnitude")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_adjustment_magnitude: Option<::Value<u32>>,
        /// Property `StepAdjustments`.
        #[serde(rename = "StepAdjustments")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub step_adjustments: Option<::ValueList<StepAdjustment>>,
    }

    cfn_internal__inherit_codec_impls!(StepScalingPolicyConfiguration);

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        /// Property `CustomizedMetricSpecification`.
        #[serde(rename = "CustomizedMetricSpecification")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub customized_metric_specification: Option<::Value<CustomizedMetricSpecification>>,
        /// Property `DisableScaleIn`.
        #[serde(rename = "DisableScaleIn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disable_scale_in: Option<::Value<bool>>,
        /// Property `PredefinedMetricSpecification`.
        #[serde(rename = "PredefinedMetricSpecification")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub predefined_metric_specification: Option<::Value<PredefinedMetricSpecification>>,
        /// Property `ScaleInCooldown`.
        #[serde(rename = "ScaleInCooldown")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scale_in_cooldown: Option<::Value<u32>>,
        /// Property `ScaleOutCooldown`.
        #[serde(rename = "ScaleOutCooldown")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scale_out_cooldown: Option<::Value<u32>>,
        /// Property `TargetValue`.
        #[serde(rename = "TargetValue")]
        pub target_value: ::Value<f64>,
    }

    cfn_internal__inherit_codec_impls!(TargetTrackingScalingPolicyConfiguration);
}
