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
    pub scheduled_actions: Vec<()>,
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
    pub step_scaling_policy_configuration: (),
    #[serde(rename="TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: (),
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
