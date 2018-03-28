/// The [`AWS::AutoScaling::AutoScalingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html) resource.
#[derive(Serialize, Deserialize)]
pub struct AutoScalingGroup {
    properties: AutoScalingGroupProperties
}

/// Properties for the `AutoScalingGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct AutoScalingGroupProperties {
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: (),
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: (),
    #[serde(rename="Cooldown")]
    pub cooldown: (),
    #[serde(rename="DesiredCapacity")]
    pub desired_capacity: (),
    #[serde(rename="HealthCheckGracePeriod")]
    pub health_check_grace_period: (),
    #[serde(rename="HealthCheckType")]
    pub health_check_type: (),
    #[serde(rename="InstanceId")]
    pub instance_id: (),
    #[serde(rename="LaunchConfigurationName")]
    pub launch_configuration_name: (),
    #[serde(rename="LifecycleHookSpecificationList")]
    pub lifecycle_hook_specification_list: (),
    #[serde(rename="LoadBalancerNames")]
    pub load_balancer_names: (),
    #[serde(rename="MaxSize")]
    pub max_size: (),
    #[serde(rename="MetricsCollection")]
    pub metrics_collection: (),
    #[serde(rename="MinSize")]
    pub min_size: (),
    #[serde(rename="NotificationConfigurations")]
    pub notification_configurations: (),
    #[serde(rename="PlacementGroup")]
    pub placement_group: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="TargetGroupARNs")]
    pub target_group_ar_ns: (),
    #[serde(rename="TerminationPolicies")]
    pub termination_policies: (),
    #[serde(rename="VPCZoneIdentifier")]
    pub vpc_zone_identifier: (),
}

impl<'a> ::Resource<'a> for AutoScalingGroup {
    type Properties = AutoScalingGroupProperties;
    const TYPE: &'static str = "AWS::AutoScaling::AutoScalingGroup";
    fn properties(&self) -> &AutoScalingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AutoScalingGroupProperties {
        &mut self.properties
    }
}

impl From<AutoScalingGroupProperties> for AutoScalingGroup {
    fn from(properties: AutoScalingGroupProperties) -> AutoScalingGroup {
        AutoScalingGroup { properties }
    }
}

/// The [`AWS::AutoScaling::LaunchConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html) resource.
#[derive(Serialize, Deserialize)]
pub struct LaunchConfiguration {
    properties: LaunchConfigurationProperties
}

/// Properties for the `LaunchConfiguration` resource.
#[derive(Serialize, Deserialize)]
pub struct LaunchConfigurationProperties {
    #[serde(rename="AssociatePublicIpAddress")]
    pub associate_public_ip_address: (),
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: (),
    #[serde(rename="ClassicLinkVPCId")]
    pub classic_link_vpc_id: (),
    #[serde(rename="ClassicLinkVPCSecurityGroups")]
    pub classic_link_vpc_security_groups: (),
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: (),
    #[serde(rename="IamInstanceProfile")]
    pub iam_instance_profile: (),
    #[serde(rename="ImageId")]
    pub image_id: (),
    #[serde(rename="InstanceId")]
    pub instance_id: (),
    #[serde(rename="InstanceMonitoring")]
    pub instance_monitoring: (),
    #[serde(rename="InstanceType")]
    pub instance_type: (),
    #[serde(rename="KernelId")]
    pub kernel_id: (),
    #[serde(rename="KeyName")]
    pub key_name: (),
    #[serde(rename="PlacementTenancy")]
    pub placement_tenancy: (),
    #[serde(rename="RamDiskId")]
    pub ram_disk_id: (),
    #[serde(rename="SecurityGroups")]
    pub security_groups: (),
    #[serde(rename="SpotPrice")]
    pub spot_price: (),
    #[serde(rename="UserData")]
    pub user_data: (),
}

impl<'a> ::Resource<'a> for LaunchConfiguration {
    type Properties = LaunchConfigurationProperties;
    const TYPE: &'static str = "AWS::AutoScaling::LaunchConfiguration";
    fn properties(&self) -> &LaunchConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchConfigurationProperties {
        &mut self.properties
    }
}

impl From<LaunchConfigurationProperties> for LaunchConfiguration {
    fn from(properties: LaunchConfigurationProperties) -> LaunchConfiguration {
        LaunchConfiguration { properties }
    }
}

/// The [`AWS::AutoScaling::LifecycleHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html) resource.
#[derive(Serialize, Deserialize)]
pub struct LifecycleHook {
    properties: LifecycleHookProperties
}

/// Properties for the `LifecycleHook` resource.
#[derive(Serialize, Deserialize)]
pub struct LifecycleHookProperties {
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: (),
    #[serde(rename="DefaultResult")]
    pub default_result: (),
    #[serde(rename="HeartbeatTimeout")]
    pub heartbeat_timeout: (),
    #[serde(rename="LifecycleHookName")]
    pub lifecycle_hook_name: (),
    #[serde(rename="LifecycleTransition")]
    pub lifecycle_transition: (),
    #[serde(rename="NotificationMetadata")]
    pub notification_metadata: (),
    #[serde(rename="NotificationTargetARN")]
    pub notification_target_arn: (),
    #[serde(rename="RoleARN")]
    pub role_arn: (),
}

impl<'a> ::Resource<'a> for LifecycleHook {
    type Properties = LifecycleHookProperties;
    const TYPE: &'static str = "AWS::AutoScaling::LifecycleHook";
    fn properties(&self) -> &LifecycleHookProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LifecycleHookProperties {
        &mut self.properties
    }
}

impl From<LifecycleHookProperties> for LifecycleHook {
    fn from(properties: LifecycleHookProperties) -> LifecycleHook {
        LifecycleHook { properties }
    }
}

/// The [`AWS::AutoScaling::ScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct ScalingPolicyProperties {
    #[serde(rename="AdjustmentType")]
    pub adjustment_type: (),
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: (),
    #[serde(rename="Cooldown")]
    pub cooldown: (),
    #[serde(rename="EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: (),
    #[serde(rename="MetricAggregationType")]
    pub metric_aggregation_type: (),
    #[serde(rename="MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: (),
    #[serde(rename="PolicyType")]
    pub policy_type: (),
    #[serde(rename="ScalingAdjustment")]
    pub scaling_adjustment: (),
    #[serde(rename="StepAdjustments")]
    pub step_adjustments: (),
    #[serde(rename="TargetTrackingConfiguration")]
    pub target_tracking_configuration: (),
}

impl<'a> ::Resource<'a> for ScalingPolicy {
    type Properties = ScalingPolicyProperties;
    const TYPE: &'static str = "AWS::AutoScaling::ScalingPolicy";
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

/// The [`AWS::AutoScaling::ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ScheduledAction {
    properties: ScheduledActionProperties
}

/// Properties for the `ScheduledAction` resource.
#[derive(Serialize, Deserialize)]
pub struct ScheduledActionProperties {
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: (),
    #[serde(rename="DesiredCapacity")]
    pub desired_capacity: (),
    #[serde(rename="EndTime")]
    pub end_time: (),
    #[serde(rename="MaxSize")]
    pub max_size: (),
    #[serde(rename="MinSize")]
    pub min_size: (),
    #[serde(rename="Recurrence")]
    pub recurrence: (),
    #[serde(rename="StartTime")]
    pub start_time: (),
}

impl<'a> ::Resource<'a> for ScheduledAction {
    type Properties = ScheduledActionProperties;
    const TYPE: &'static str = "AWS::AutoScaling::ScheduledAction";
    fn properties(&self) -> &ScheduledActionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduledActionProperties {
        &mut self.properties
    }
}

impl From<ScheduledActionProperties> for ScheduledAction {
    fn from(properties: ScheduledActionProperties) -> ScheduledAction {
        ScheduledAction { properties }
    }
}

