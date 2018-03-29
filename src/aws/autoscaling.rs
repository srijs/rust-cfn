/// The [`AWS::AutoScaling::AutoScalingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html) resource.
pub struct AutoScalingGroup {
    properties: AutoScalingGroupProperties
}

/// Properties for the `AutoScalingGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct AutoScalingGroupProperties {
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    #[serde(rename="Cooldown")]
    pub cooldown: String,
    #[serde(rename="DesiredCapacity")]
    pub desired_capacity: String,
    #[serde(rename="HealthCheckGracePeriod")]
    pub health_check_grace_period: u32,
    #[serde(rename="HealthCheckType")]
    pub health_check_type: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="LaunchConfigurationName")]
    pub launch_configuration_name: String,
    #[serde(rename="LifecycleHookSpecificationList")]
    pub lifecycle_hook_specification_list: Vec<self::auto_scaling_group::LifecycleHookSpecification>,
    #[serde(rename="LoadBalancerNames")]
    pub load_balancer_names: Vec<String>,
    #[serde(rename="MaxSize")]
    pub max_size: String,
    #[serde(rename="MetricsCollection")]
    pub metrics_collection: Vec<self::auto_scaling_group::MetricsCollection>,
    #[serde(rename="MinSize")]
    pub min_size: String,
    #[serde(rename="NotificationConfigurations")]
    pub notification_configurations: Vec<self::auto_scaling_group::NotificationConfiguration>,
    #[serde(rename="PlacementGroup")]
    pub placement_group: String,
    #[serde(rename="Tags")]
    pub tags: Vec<self::auto_scaling_group::TagProperty>,
    #[serde(rename="TargetGroupARNs")]
    pub target_group_ar_ns: Vec<String>,
    #[serde(rename="TerminationPolicies")]
    pub termination_policies: Vec<String>,
    #[serde(rename="VPCZoneIdentifier")]
    pub vpc_zone_identifier: Vec<String>,
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
pub struct LaunchConfiguration {
    properties: LaunchConfigurationProperties
}

/// Properties for the `LaunchConfiguration` resource.
#[derive(Serialize, Deserialize)]
pub struct LaunchConfigurationProperties {
    #[serde(rename="AssociatePublicIpAddress")]
    pub associate_public_ip_address: bool,
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: Vec<self::launch_configuration::BlockDeviceMapping>,
    #[serde(rename="ClassicLinkVPCId")]
    pub classic_link_vpc_id: String,
    #[serde(rename="ClassicLinkVPCSecurityGroups")]
    pub classic_link_vpc_security_groups: Vec<String>,
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: bool,
    #[serde(rename="IamInstanceProfile")]
    pub iam_instance_profile: String,
    #[serde(rename="ImageId")]
    pub image_id: String,
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[serde(rename="InstanceMonitoring")]
    pub instance_monitoring: bool,
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[serde(rename="KernelId")]
    pub kernel_id: String,
    #[serde(rename="KeyName")]
    pub key_name: String,
    #[serde(rename="PlacementTenancy")]
    pub placement_tenancy: String,
    #[serde(rename="RamDiskId")]
    pub ram_disk_id: String,
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename="SpotPrice")]
    pub spot_price: String,
    #[serde(rename="UserData")]
    pub user_data: String,
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
pub struct LifecycleHook {
    properties: LifecycleHookProperties
}

/// Properties for the `LifecycleHook` resource.
#[derive(Serialize, Deserialize)]
pub struct LifecycleHookProperties {
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    #[serde(rename="DefaultResult")]
    pub default_result: String,
    #[serde(rename="HeartbeatTimeout")]
    pub heartbeat_timeout: u32,
    #[serde(rename="LifecycleHookName")]
    pub lifecycle_hook_name: String,
    #[serde(rename="LifecycleTransition")]
    pub lifecycle_transition: String,
    #[serde(rename="NotificationMetadata")]
    pub notification_metadata: String,
    #[serde(rename="NotificationTargetARN")]
    pub notification_target_arn: String,
    #[serde(rename="RoleARN")]
    pub role_arn: String,
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
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct ScalingPolicyProperties {
    #[serde(rename="AdjustmentType")]
    pub adjustment_type: String,
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    #[serde(rename="Cooldown")]
    pub cooldown: String,
    #[serde(rename="EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: u32,
    #[serde(rename="MetricAggregationType")]
    pub metric_aggregation_type: String,
    #[serde(rename="MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: u32,
    #[serde(rename="PolicyType")]
    pub policy_type: String,
    #[serde(rename="ScalingAdjustment")]
    pub scaling_adjustment: u32,
    #[serde(rename="StepAdjustments")]
    pub step_adjustments: Vec<self::scaling_policy::StepAdjustment>,
    #[serde(rename="TargetTrackingConfiguration")]
    pub target_tracking_configuration: self::scaling_policy::TargetTrackingConfiguration,
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
pub struct ScheduledAction {
    properties: ScheduledActionProperties
}

/// Properties for the `ScheduledAction` resource.
#[derive(Serialize, Deserialize)]
pub struct ScheduledActionProperties {
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    #[serde(rename="DesiredCapacity")]
    pub desired_capacity: u32,
    #[serde(rename="EndTime")]
    pub end_time: String,
    #[serde(rename="MaxSize")]
    pub max_size: u32,
    #[serde(rename="MinSize")]
    pub min_size: u32,
    #[serde(rename="Recurrence")]
    pub recurrence: String,
    #[serde(rename="StartTime")]
    pub start_time: String,
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

pub mod auto_scaling_group {
    #[derive(Serialize, Deserialize)]
    pub struct LifecycleHookSpecification {
        #[serde(rename="DefaultResult")]
        pub default_result: String,
        #[serde(rename="HeartbeatTimeout")]
        pub heartbeat_timeout: u32,
        #[serde(rename="LifecycleHookName")]
        pub lifecycle_hook_name: String,
        #[serde(rename="LifecycleTransition")]
        pub lifecycle_transition: String,
        #[serde(rename="NotificationMetadata")]
        pub notification_metadata: String,
        #[serde(rename="NotificationTargetARN")]
        pub notification_target_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetricsCollection {
        #[serde(rename="Granularity")]
        pub granularity: String,
        #[serde(rename="Metrics")]
        pub metrics: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        #[serde(rename="NotificationTypes")]
        pub notification_types: Vec<String>,
        #[serde(rename="TopicARN")]
        pub topic_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TagProperty {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="PropagateAtLaunch")]
        pub propagate_at_launch: bool,
        #[serde(rename="Value")]
        pub value: String,
    }

}

pub mod launch_configuration {
    #[derive(Serialize, Deserialize)]
    pub struct BlockDevice {
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        #[serde(rename="Encrypted")]
        pub encrypted: bool,
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SnapshotId")]
        pub snapshot_id: String,
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        #[serde(rename="DeviceName")]
        pub device_name: String,
        #[serde(rename="Ebs")]
        pub ebs: BlockDevice,
        #[serde(rename="NoDevice")]
        pub no_device: bool,
        #[serde(rename="VirtualName")]
        pub virtual_name: String,
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
    pub struct TargetTrackingConfiguration {
        #[serde(rename="CustomizedMetricSpecification")]
        pub customized_metric_specification: CustomizedMetricSpecification,
        #[serde(rename="DisableScaleIn")]
        pub disable_scale_in: bool,
        #[serde(rename="PredefinedMetricSpecification")]
        pub predefined_metric_specification: PredefinedMetricSpecification,
        #[serde(rename="TargetValue")]
        pub target_value: f64,
    }

}

