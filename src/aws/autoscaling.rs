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
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<::Value<String>>,
    /// Property `AvailabilityZones`.
    #[serde(rename = "AvailabilityZones")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `Cooldown`.
    #[serde(rename = "Cooldown")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<::Value<String>>,
    /// Property `DesiredCapacity`.
    #[serde(rename = "DesiredCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<::Value<String>>,
    /// Property `HealthCheckGracePeriod`.
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<::Value<u32>>,
    /// Property `HealthCheckType`.
    #[serde(rename = "HealthCheckType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<::Value<String>>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
    /// Property `LaunchConfigurationName`.
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<::Value<String>>,
    /// Property `LifecycleHookSpecificationList`.
    #[serde(rename = "LifecycleHookSpecificationList")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_specification_list: Option<::ValueList<self::auto_scaling_group::LifecycleHookSpecification>>,
    /// Property `LoadBalancerNames`.
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<::ValueList<String>>,
    /// Property `MaxSize`.
    #[serde(rename = "MaxSize")]
    pub max_size: ::Value<String>,
    /// Property `MetricsCollection`.
    #[serde(rename = "MetricsCollection")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics_collection: Option<::ValueList<self::auto_scaling_group::MetricsCollection>>,
    /// Property `MinSize`.
    #[serde(rename = "MinSize")]
    pub min_size: ::Value<String>,
    /// Property `NotificationConfigurations`.
    #[serde(rename = "NotificationConfigurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_configurations: Option<::ValueList<self::auto_scaling_group::NotificationConfiguration>>,
    /// Property `PlacementGroup`.
    #[serde(rename = "PlacementGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<self::auto_scaling_group::TagProperty>>,
    /// Property `TargetGroupARNs`.
    #[serde(rename = "TargetGroupARNs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_group_ar_ns: Option<::ValueList<String>>,
    /// Property `TerminationPolicies`.
    #[serde(rename = "TerminationPolicies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_policies: Option<::ValueList<String>>,
    /// Property `VPCZoneIdentifier`.
    #[serde(rename = "VPCZoneIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_zone_identifier: Option<::ValueList<String>>,
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
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<::Value<bool>>,
    /// Property `BlockDeviceMappings`.
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<::ValueList<self::launch_configuration::BlockDeviceMapping>>,
    /// Property `ClassicLinkVPCId`.
    #[serde(rename = "ClassicLinkVPCId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classic_link_vpc_id: Option<::Value<String>>,
    /// Property `ClassicLinkVPCSecurityGroups`.
    #[serde(rename = "ClassicLinkVPCSecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classic_link_vpc_security_groups: Option<::ValueList<String>>,
    /// Property `EbsOptimized`.
    #[serde(rename = "EbsOptimized")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property `IamInstanceProfile`.
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<::Value<String>>,
    /// Property `ImageId`.
    #[serde(rename = "ImageId")]
    pub image_id: ::Value<String>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
    /// Property `InstanceMonitoring`.
    #[serde(rename = "InstanceMonitoring")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_monitoring: Option<::Value<bool>>,
    /// Property `InstanceType`.
    #[serde(rename = "InstanceType")]
    pub instance_type: ::Value<String>,
    /// Property `KernelId`.
    #[serde(rename = "KernelId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<::Value<String>>,
    /// Property `KeyName`.
    #[serde(rename = "KeyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<::Value<String>>,
    /// Property `PlacementTenancy`.
    #[serde(rename = "PlacementTenancy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_tenancy: Option<::Value<String>>,
    /// Property `RamDiskId`.
    #[serde(rename = "RamDiskId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ram_disk_id: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    #[serde(rename = "SecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<::ValueList<String>>,
    /// Property `SpotPrice`.
    #[serde(rename = "SpotPrice")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<::Value<String>>,
    /// Property `UserData`.
    #[serde(rename = "UserData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<::Value<String>>,
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
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: ::Value<String>,
    /// Property `DefaultResult`.
    #[serde(rename = "DefaultResult")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_result: Option<::Value<String>>,
    /// Property `HeartbeatTimeout`.
    #[serde(rename = "HeartbeatTimeout")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<::Value<u32>>,
    /// Property `LifecycleHookName`.
    #[serde(rename = "LifecycleHookName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_hook_name: Option<::Value<String>>,
    /// Property `LifecycleTransition`.
    #[serde(rename = "LifecycleTransition")]
    pub lifecycle_transition: ::Value<String>,
    /// Property `NotificationMetadata`.
    #[serde(rename = "NotificationMetadata")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_metadata: Option<::Value<String>>,
    /// Property `NotificationTargetARN`.
    #[serde(rename = "NotificationTargetARN")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_target_arn: Option<::Value<String>>,
    /// Property `RoleARN`.
    #[serde(rename = "RoleARN")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<::Value<String>>,
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
    #[serde(rename = "AdjustmentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<::Value<String>>,
    /// Property `AutoScalingGroupName`.
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: ::Value<String>,
    /// Property `Cooldown`.
    #[serde(rename = "Cooldown")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<::Value<String>>,
    /// Property `EstimatedInstanceWarmup`.
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<::Value<u32>>,
    /// Property `MetricAggregationType`.
    #[serde(rename = "MetricAggregationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<::Value<String>>,
    /// Property `MinAdjustmentMagnitude`.
    #[serde(rename = "MinAdjustmentMagnitude")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<::Value<u32>>,
    /// Property `PolicyType`.
    #[serde(rename = "PolicyType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<::Value<String>>,
    /// Property `ScalingAdjustment`.
    #[serde(rename = "ScalingAdjustment")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment: Option<::Value<u32>>,
    /// Property `StepAdjustments`.
    #[serde(rename = "StepAdjustments")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<::ValueList<self::scaling_policy::StepAdjustment>>,
    /// Property `TargetTrackingConfiguration`.
    #[serde(rename = "TargetTrackingConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tracking_configuration: Option<::Value<self::scaling_policy::TargetTrackingConfiguration>>,
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
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: ::Value<String>,
    /// Property `DesiredCapacity`.
    #[serde(rename = "DesiredCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<::Value<u32>>,
    /// Property `EndTime`.
    #[serde(rename = "EndTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<::Value<String>>,
    /// Property `MaxSize`.
    #[serde(rename = "MaxSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<::Value<u32>>,
    /// Property `MinSize`.
    #[serde(rename = "MinSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<::Value<u32>>,
    /// Property `Recurrence`.
    #[serde(rename = "Recurrence")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<::Value<String>>,
    /// Property `StartTime`.
    #[serde(rename = "StartTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<::Value<String>>,
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
        #[serde(rename = "DefaultResult")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_result: Option<::Value<String>>,
        /// Property `HeartbeatTimeout`.
        #[serde(rename = "HeartbeatTimeout")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub heartbeat_timeout: Option<::Value<u32>>,
        /// Property `LifecycleHookName`.
        #[serde(rename = "LifecycleHookName")]
        pub lifecycle_hook_name: ::Value<String>,
        /// Property `LifecycleTransition`.
        #[serde(rename = "LifecycleTransition")]
        pub lifecycle_transition: ::Value<String>,
        /// Property `NotificationMetadata`.
        #[serde(rename = "NotificationMetadata")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notification_metadata: Option<::Value<String>>,
        /// Property `NotificationTargetARN`.
        #[serde(rename = "NotificationTargetARN")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notification_target_arn: Option<::Value<String>>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LifecycleHookSpecification);

    /// The [`AWS::AutoScaling::AutoScalingGroup.MetricsCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricsCollection {
        /// Property `Granularity`.
        #[serde(rename = "Granularity")]
        pub granularity: ::Value<String>,
        /// Property `Metrics`.
        #[serde(rename = "Metrics")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metrics: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(MetricsCollection);

    /// The [`AWS::AutoScaling::AutoScalingGroup.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        /// Property `NotificationTypes`.
        #[serde(rename = "NotificationTypes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notification_types: Option<::ValueList<String>>,
        /// Property `TopicARN`.
        #[serde(rename = "TopicARN")]
        pub topic_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(NotificationConfiguration);

    /// The [`AWS::AutoScaling::AutoScalingGroup.TagProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TagProperty {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `PropagateAtLaunch`.
        #[serde(rename = "PropagateAtLaunch")]
        pub propagate_at_launch: ::Value<bool>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(TagProperty);
}

pub mod launch_configuration {
    //! Property types for the `LaunchConfiguration` resource.

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDevice {
        /// Property `DeleteOnTermination`.
        #[serde(rename = "DeleteOnTermination")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Encrypted`.
        #[serde(rename = "Encrypted")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<::Value<bool>>,
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        #[serde(rename = "SnapshotId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        #[serde(rename = "VolumeSize")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(BlockDevice);

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename = "DeviceName")]
        pub device_name: ::Value<String>,
        /// Property `Ebs`.
        #[serde(rename = "Ebs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs: Option<::Value<BlockDevice>>,
        /// Property `NoDevice`.
        #[serde(rename = "NoDevice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub no_device: Option<::Value<bool>>,
        /// Property `VirtualName`.
        #[serde(rename = "VirtualName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub virtual_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(BlockDeviceMapping);
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::AutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html) property type.
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

    /// The [`AWS::AutoScaling::ScalingPolicy.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetTrackingConfiguration {
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
        /// Property `TargetValue`.
        #[serde(rename = "TargetValue")]
        pub target_value: ::Value<f64>,
    }

    cfn_internal__inherit_codec_impls!(TargetTrackingConfiguration);
}
