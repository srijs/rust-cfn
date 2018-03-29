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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    /// Property `AvailabilityZones`.
    #[serde(rename="AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// Property `Cooldown`.
    #[serde(rename="Cooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<String>,
    /// Property `DesiredCapacity`.
    #[serde(rename="DesiredCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<String>,
    /// Property `HealthCheckGracePeriod`.
    #[serde(rename="HealthCheckGracePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<u32>,
    /// Property `HealthCheckType`.
    #[serde(rename="HealthCheckType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    /// Property `InstanceId`.
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Property `LaunchConfigurationName`.
    #[serde(rename="LaunchConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    /// Property `LifecycleHookSpecificationList`.
    #[serde(rename="LifecycleHookSpecificationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_specification_list: Option<Vec<self::auto_scaling_group::LifecycleHookSpecification>>,
    /// Property `LoadBalancerNames`.
    #[serde(rename="LoadBalancerNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<Vec<String>>,
    /// Property `MaxSize`.
    #[serde(rename="MaxSize")]
    pub max_size: String,
    /// Property `MetricsCollection`.
    #[serde(rename="MetricsCollection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_collection: Option<Vec<self::auto_scaling_group::MetricsCollection>>,
    /// Property `MinSize`.
    #[serde(rename="MinSize")]
    pub min_size: String,
    /// Property `NotificationConfigurations`.
    #[serde(rename="NotificationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configurations: Option<Vec<self::auto_scaling_group::NotificationConfiguration>>,
    /// Property `PlacementGroup`.
    #[serde(rename="PlacementGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<self::auto_scaling_group::TagProperty>>,
    /// Property `TargetGroupARNs`.
    #[serde(rename="TargetGroupARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_ar_ns: Option<Vec<String>>,
    /// Property `TerminationPolicies`.
    #[serde(rename="TerminationPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_policies: Option<Vec<String>>,
    /// Property `VPCZoneIdentifier`.
    #[serde(rename="VPCZoneIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_zone_identifier: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    /// Property `BlockDeviceMappings`.
    #[serde(rename="BlockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<self::launch_configuration::BlockDeviceMapping>>,
    /// Property `ClassicLinkVPCId`.
    #[serde(rename="ClassicLinkVPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_vpc_id: Option<String>,
    /// Property `ClassicLinkVPCSecurityGroups`.
    #[serde(rename="ClassicLinkVPCSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// Property `EbsOptimized`.
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// Property `IamInstanceProfile`.
    #[serde(rename="IamInstanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,
    /// Property `ImageId`.
    #[serde(rename="ImageId")]
    pub image_id: String,
    /// Property `InstanceId`.
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Property `InstanceMonitoring`.
    #[serde(rename="InstanceMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_monitoring: Option<bool>,
    /// Property `InstanceType`.
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    /// Property `KernelId`.
    #[serde(rename="KernelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,
    /// Property `KeyName`.
    #[serde(rename="KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// Property `PlacementTenancy`.
    #[serde(rename="PlacementTenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_tenancy: Option<String>,
    /// Property `RamDiskId`.
    #[serde(rename="RamDiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_disk_id: Option<String>,
    /// Property `SecurityGroups`.
    #[serde(rename="SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// Property `SpotPrice`.
    #[serde(rename="SpotPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<String>,
    /// Property `UserData`.
    #[serde(rename="UserData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_result: Option<String>,
    /// Property `HeartbeatTimeout`.
    #[serde(rename="HeartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<u32>,
    /// Property `LifecycleHookName`.
    #[serde(rename="LifecycleHookName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_name: Option<String>,
    /// Property `LifecycleTransition`.
    #[serde(rename="LifecycleTransition")]
    pub lifecycle_transition: String,
    /// Property `NotificationMetadata`.
    #[serde(rename="NotificationMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_metadata: Option<String>,
    /// Property `NotificationTargetARN`.
    #[serde(rename="NotificationTargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target_arn: Option<String>,
    /// Property `RoleARN`.
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    /// Property `AutoScalingGroupName`.
    #[serde(rename="AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// Property `Cooldown`.
    #[serde(rename="Cooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<String>,
    /// Property `EstimatedInstanceWarmup`.
    #[serde(rename="EstimatedInstanceWarmup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<u32>,
    /// Property `MetricAggregationType`.
    #[serde(rename="MetricAggregationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    /// Property `MinAdjustmentMagnitude`.
    #[serde(rename="MinAdjustmentMagnitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<u32>,
    /// Property `PolicyType`.
    #[serde(rename="PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// Property `ScalingAdjustment`.
    #[serde(rename="ScalingAdjustment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment: Option<u32>,
    /// Property `StepAdjustments`.
    #[serde(rename="StepAdjustments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<Vec<self::scaling_policy::StepAdjustment>>,
    /// Property `TargetTrackingConfiguration`.
    #[serde(rename="TargetTrackingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_configuration: Option<self::scaling_policy::TargetTrackingConfiguration>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<u32>,
    /// Property `EndTime`.
    #[serde(rename="EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Property `MaxSize`.
    #[serde(rename="MaxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,
    /// Property `MinSize`.
    #[serde(rename="MinSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u32>,
    /// Property `Recurrence`.
    #[serde(rename="Recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    /// Property `StartTime`.
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_result: Option<String>,
        /// Property `HeartbeatTimeout`.
        #[serde(rename="HeartbeatTimeout")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub heartbeat_timeout: Option<u32>,
        /// Property `LifecycleHookName`.
        #[serde(rename="LifecycleHookName")]
        pub lifecycle_hook_name: String,
        /// Property `LifecycleTransition`.
        #[serde(rename="LifecycleTransition")]
        pub lifecycle_transition: String,
        /// Property `NotificationMetadata`.
        #[serde(rename="NotificationMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notification_metadata: Option<String>,
        /// Property `NotificationTargetARN`.
        #[serde(rename="NotificationTargetARN")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notification_target_arn: Option<String>,
        /// Property `RoleARN`.
        #[serde(rename="RoleARN")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<String>,
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.MetricsCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricsCollection {
        /// Property `Granularity`.
        #[serde(rename="Granularity")]
        pub granularity: String,
        /// Property `Metrics`.
        #[serde(rename="Metrics")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metrics: Option<Vec<String>>,
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        /// Property `NotificationTypes`.
        #[serde(rename="NotificationTypes")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notification_types: Option<Vec<String>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<bool>,
        /// Property `Encrypted`.
        #[serde(rename="Encrypted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<bool>,
        /// Property `Iops`.
        #[serde(rename="Iops")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iops: Option<u32>,
        /// Property `SnapshotId`.
        #[serde(rename="SnapshotId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<String>,
        /// Property `VolumeSize`.
        #[serde(rename="VolumeSize")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<u32>,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<String>,
    }

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename="DeviceName")]
        pub device_name: String,
        /// Property `Ebs`.
        #[serde(rename="Ebs")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ebs: Option<BlockDevice>,
        /// Property `NoDevice`.
        #[serde(rename="NoDevice")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub no_device: Option<bool>,
        /// Property `VirtualName`.
        #[serde(rename="VirtualName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub virtual_name: Option<String>,
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::AutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html) property type.
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_label: Option<String>,
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetTrackingConfiguration {
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
        /// Property `TargetValue`.
        #[serde(rename="TargetValue")]
        pub target_value: f64,
    }
}
