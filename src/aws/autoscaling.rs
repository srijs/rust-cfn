//! Types for the `AutoScaling` service.

/// The [`AWS::AutoScaling::AutoScalingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html) resource type.
#[derive(Debug)]
pub struct AutoScalingGroup {
    properties: AutoScalingGroupProperties
}

/// Properties for the `AutoScalingGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoScalingGroupProperties {
    /// Property `AutoScalingGroupName`.
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// Property `AvailabilityZones`.
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    /// Property `Cooldown`.
    #[serde(rename="Cooldown")]
    pub cooldown: String,
    /// Property `DesiredCapacity`.
    #[serde(rename="DesiredCapacity")]
    pub desired_capacity: String,
    /// Property `HealthCheckGracePeriod`.
    #[serde(rename="HealthCheckGracePeriod")]
    pub health_check_grace_period: u32,
    /// Property `HealthCheckType`.
    #[serde(rename="HealthCheckType")]
    pub health_check_type: String,
    /// Property `InstanceId`.
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    /// Property `LaunchConfigurationName`.
    #[serde(rename="LaunchConfigurationName")]
    pub launch_configuration_name: String,
    /// Property `LifecycleHookSpecificationList`.
    #[serde(rename="LifecycleHookSpecificationList")]
    pub lifecycle_hook_specification_list: Vec<self::auto_scaling_group::LifecycleHookSpecification>,
    /// Property `LoadBalancerNames`.
    #[serde(rename="LoadBalancerNames")]
    pub load_balancer_names: Vec<String>,
    /// Property `MaxSize`.
    #[serde(rename="MaxSize")]
    pub max_size: String,
    /// Property `MetricsCollection`.
    #[serde(rename="MetricsCollection")]
    pub metrics_collection: Vec<self::auto_scaling_group::MetricsCollection>,
    /// Property `MinSize`.
    #[serde(rename="MinSize")]
    pub min_size: String,
    /// Property `NotificationConfigurations`.
    #[serde(rename="NotificationConfigurations")]
    pub notification_configurations: Vec<self::auto_scaling_group::NotificationConfiguration>,
    /// Property `PlacementGroup`.
    #[serde(rename="PlacementGroup")]
    pub placement_group: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: Vec<self::auto_scaling_group::TagProperty>,
    /// Property `TargetGroupARNs`.
    #[serde(rename="TargetGroupARNs")]
    pub target_group_ar_ns: Vec<String>,
    /// Property `TerminationPolicies`.
    #[serde(rename="TerminationPolicies")]
    pub termination_policies: Vec<String>,
    /// Property `VPCZoneIdentifier`.
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

impl ::private::Sealed for AutoScalingGroup {}

impl From<AutoScalingGroupProperties> for AutoScalingGroup {
    fn from(properties: AutoScalingGroupProperties) -> AutoScalingGroup {
        AutoScalingGroup { properties }
    }
}

/// The [`AWS::AutoScaling::LaunchConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig.html) resource type.
#[derive(Debug)]
pub struct LaunchConfiguration {
    properties: LaunchConfigurationProperties
}

/// Properties for the `LaunchConfiguration` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchConfigurationProperties {
    /// Property `AssociatePublicIpAddress`.
    #[serde(rename="AssociatePublicIpAddress")]
    pub associate_public_ip_address: bool,
    /// Property `BlockDeviceMappings`.
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: Vec<self::launch_configuration::BlockDeviceMapping>,
    /// Property `ClassicLinkVPCId`.
    #[serde(rename="ClassicLinkVPCId")]
    pub classic_link_vpc_id: String,
    /// Property `ClassicLinkVPCSecurityGroups`.
    #[serde(rename="ClassicLinkVPCSecurityGroups")]
    pub classic_link_vpc_security_groups: Vec<String>,
    /// Property `EbsOptimized`.
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: bool,
    /// Property `IamInstanceProfile`.
    #[serde(rename="IamInstanceProfile")]
    pub iam_instance_profile: String,
    /// Property `ImageId`.
    #[serde(rename="ImageId")]
    pub image_id: String,
    /// Property `InstanceId`.
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    /// Property `InstanceMonitoring`.
    #[serde(rename="InstanceMonitoring")]
    pub instance_monitoring: bool,
    /// Property `InstanceType`.
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    /// Property `KernelId`.
    #[serde(rename="KernelId")]
    pub kernel_id: String,
    /// Property `KeyName`.
    #[serde(rename="KeyName")]
    pub key_name: String,
    /// Property `PlacementTenancy`.
    #[serde(rename="PlacementTenancy")]
    pub placement_tenancy: String,
    /// Property `RamDiskId`.
    #[serde(rename="RamDiskId")]
    pub ram_disk_id: String,
    /// Property `SecurityGroups`.
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    /// Property `SpotPrice`.
    #[serde(rename="SpotPrice")]
    pub spot_price: String,
    /// Property `UserData`.
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

impl ::private::Sealed for LaunchConfiguration {}

impl From<LaunchConfigurationProperties> for LaunchConfiguration {
    fn from(properties: LaunchConfigurationProperties) -> LaunchConfiguration {
        LaunchConfiguration { properties }
    }
}

/// The [`AWS::AutoScaling::LifecycleHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-lifecyclehook.html) resource type.
#[derive(Debug)]
pub struct LifecycleHook {
    properties: LifecycleHookProperties
}

/// Properties for the `LifecycleHook` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleHookProperties {
    /// Property `AutoScalingGroupName`.
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// Property `DefaultResult`.
    #[serde(rename="DefaultResult")]
    pub default_result: String,
    /// Property `HeartbeatTimeout`.
    #[serde(rename="HeartbeatTimeout")]
    pub heartbeat_timeout: u32,
    /// Property `LifecycleHookName`.
    #[serde(rename="LifecycleHookName")]
    pub lifecycle_hook_name: String,
    /// Property `LifecycleTransition`.
    #[serde(rename="LifecycleTransition")]
    pub lifecycle_transition: String,
    /// Property `NotificationMetadata`.
    #[serde(rename="NotificationMetadata")]
    pub notification_metadata: String,
    /// Property `NotificationTargetARN`.
    #[serde(rename="NotificationTargetARN")]
    pub notification_target_arn: String,
    /// Property `RoleARN`.
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

impl ::private::Sealed for LifecycleHook {}

impl From<LifecycleHookProperties> for LifecycleHook {
    fn from(properties: LifecycleHookProperties) -> LifecycleHook {
        LifecycleHook { properties }
    }
}

/// The [`AWS::AutoScaling::ScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-policy.html) resource type.
#[derive(Debug)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingPolicyProperties {
    /// Property `AdjustmentType`.
    #[serde(rename="AdjustmentType")]
    pub adjustment_type: String,
    /// Property `AutoScalingGroupName`.
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// Property `Cooldown`.
    #[serde(rename="Cooldown")]
    pub cooldown: String,
    /// Property `EstimatedInstanceWarmup`.
    #[serde(rename="EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: u32,
    /// Property `MetricAggregationType`.
    #[serde(rename="MetricAggregationType")]
    pub metric_aggregation_type: String,
    /// Property `MinAdjustmentMagnitude`.
    #[serde(rename="MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: u32,
    /// Property `PolicyType`.
    #[serde(rename="PolicyType")]
    pub policy_type: String,
    /// Property `ScalingAdjustment`.
    #[serde(rename="ScalingAdjustment")]
    pub scaling_adjustment: u32,
    /// Property `StepAdjustments`.
    #[serde(rename="StepAdjustments")]
    pub step_adjustments: Vec<self::scaling_policy::StepAdjustment>,
    /// Property `TargetTrackingConfiguration`.
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

impl ::private::Sealed for ScalingPolicy {}

impl From<ScalingPolicyProperties> for ScalingPolicy {
    fn from(properties: ScalingPolicyProperties) -> ScalingPolicy {
        ScalingPolicy { properties }
    }
}

/// The [`AWS::AutoScaling::ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-as-scheduledaction.html) resource type.
#[derive(Debug)]
pub struct ScheduledAction {
    properties: ScheduledActionProperties
}

/// Properties for the `ScheduledAction` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledActionProperties {
    /// Property `AutoScalingGroupName`.
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// Property `DesiredCapacity`.
    #[serde(rename="DesiredCapacity")]
    pub desired_capacity: u32,
    /// Property `EndTime`.
    #[serde(rename="EndTime")]
    pub end_time: String,
    /// Property `MaxSize`.
    #[serde(rename="MaxSize")]
    pub max_size: u32,
    /// Property `MinSize`.
    #[serde(rename="MinSize")]
    pub min_size: u32,
    /// Property `Recurrence`.
    #[serde(rename="Recurrence")]
    pub recurrence: String,
    /// Property `StartTime`.
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

impl ::private::Sealed for ScheduledAction {}

impl From<ScheduledActionProperties> for ScheduledAction {
    fn from(properties: ScheduledActionProperties) -> ScheduledAction {
        ScheduledAction { properties }
    }
}

pub mod auto_scaling_group {
    //! Property types for the `AutoScalingGroup` resource.

    /// The [`AWS::AutoScaling::AutoScalingGroup.LifecycleHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecycleHookSpecification {
        /// Property `DefaultResult`.
        #[serde(rename="DefaultResult")]
        pub default_result: String,
        /// Property `HeartbeatTimeout`.
        #[serde(rename="HeartbeatTimeout")]
        pub heartbeat_timeout: u32,
        /// Property `LifecycleHookName`.
        #[serde(rename="LifecycleHookName")]
        pub lifecycle_hook_name: String,
        /// Property `LifecycleTransition`.
        #[serde(rename="LifecycleTransition")]
        pub lifecycle_transition: String,
        /// Property `NotificationMetadata`.
        #[serde(rename="NotificationMetadata")]
        pub notification_metadata: String,
        /// Property `NotificationTargetARN`.
        #[serde(rename="NotificationTargetARN")]
        pub notification_target_arn: String,
        /// Property `RoleARN`.
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.MetricsCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricsCollection {
        /// Property `Granularity`.
        #[serde(rename="Granularity")]
        pub granularity: String,
        /// Property `Metrics`.
        #[serde(rename="Metrics")]
        pub metrics: Vec<String>,
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        /// Property `NotificationTypes`.
        #[serde(rename="NotificationTypes")]
        pub notification_types: Vec<String>,
        /// Property `TopicARN`.
        #[serde(rename="TopicARN")]
        pub topic_arn: String,
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.TagProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TagProperty {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `PropagateAtLaunch`.
        #[serde(rename="PropagateAtLaunch")]
        pub propagate_at_launch: bool,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}

pub mod launch_configuration {
    //! Property types for the `LaunchConfiguration` resource.

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDevice {
        /// Property `DeleteOnTermination`.
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        /// Property `Encrypted`.
        #[serde(rename="Encrypted")]
        pub encrypted: bool,
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `SnapshotId`.
        #[serde(rename="SnapshotId")]
        pub snapshot_id: String,
        /// Property `VolumeSize`.
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename="DeviceName")]
        pub device_name: String,
        /// Property `Ebs`.
        #[serde(rename="Ebs")]
        pub ebs: BlockDevice,
        /// Property `NoDevice`.
        #[serde(rename="NoDevice")]
        pub no_device: bool,
        /// Property `VirtualName`.
        #[serde(rename="VirtualName")]
        pub virtual_name: String,
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::AutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PredefinedMetricSpecification {
        /// Property `PredefinedMetricType`.
        #[serde(rename="PredefinedMetricType")]
        pub predefined_metric_type: String,
        /// Property `ResourceLabel`.
        #[serde(rename="ResourceLabel")]
        pub resource_label: String,
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetTrackingConfiguration {
        /// Property `CustomizedMetricSpecification`.
        #[serde(rename="CustomizedMetricSpecification")]
        pub customized_metric_specification: CustomizedMetricSpecification,
        /// Property `DisableScaleIn`.
        #[serde(rename="DisableScaleIn")]
        pub disable_scale_in: bool,
        /// Property `PredefinedMetricSpecification`.
        #[serde(rename="PredefinedMetricSpecification")]
        pub predefined_metric_specification: PredefinedMetricSpecification,
        /// Property `TargetValue`.
        #[serde(rename="TargetValue")]
        pub target_value: f64,
    }
}
