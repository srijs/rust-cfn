//! Types for the `AutoScaling` service.

/// The [`AWS::AutoScaling::AutoScalingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-group.html) resource type.
#[derive(Debug)]
pub struct AutoScalingGroup {
    properties: AutoScalingGroupProperties
}

/// Properties for the `AutoScalingGroup` resource.
#[derive(Debug)]
pub struct AutoScalingGroupProperties {
    /// Property `AutoScalingGroupName`.
    pub auto_scaling_group_name: Option<::Value<String>>,
    /// Property `AvailabilityZones`.
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `Cooldown`.
    pub cooldown: Option<::Value<String>>,
    /// Property `DesiredCapacity`.
    pub desired_capacity: Option<::Value<String>>,
    /// Property `HealthCheckGracePeriod`.
    pub health_check_grace_period: Option<::Value<u32>>,
    /// Property `HealthCheckType`.
    pub health_check_type: Option<::Value<String>>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
    /// Property `LaunchConfigurationName`.
    pub launch_configuration_name: Option<::Value<String>>,
    /// Property `LifecycleHookSpecificationList`.
    pub lifecycle_hook_specification_list: Option<::ValueList<self::auto_scaling_group::LifecycleHookSpecification>>,
    /// Property `LoadBalancerNames`.
    pub load_balancer_names: Option<::ValueList<String>>,
    /// Property `MaxSize`.
    pub max_size: ::Value<String>,
    /// Property `MetricsCollection`.
    pub metrics_collection: Option<::ValueList<self::auto_scaling_group::MetricsCollection>>,
    /// Property `MinSize`.
    pub min_size: ::Value<String>,
    /// Property `NotificationConfigurations`.
    pub notification_configurations: Option<::ValueList<self::auto_scaling_group::NotificationConfiguration>>,
    /// Property `PlacementGroup`.
    pub placement_group: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<self::auto_scaling_group::TagProperty>>,
    /// Property `TargetGroupARNs`.
    pub target_group_ar_ns: Option<::ValueList<String>>,
    /// Property `TerminationPolicies`.
    pub termination_policies: Option<::ValueList<String>>,
    /// Property `VPCZoneIdentifier`.
    pub vpc_zone_identifier: Option<::ValueList<String>>,
}

impl ::serde::Serialize for AutoScalingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", &self.availability_zones)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cooldown", &self.cooldown)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredCapacity", &self.desired_capacity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckGracePeriod", &self.health_check_grace_period)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckType", &self.health_check_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchConfigurationName", &self.launch_configuration_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleHookSpecificationList", &self.lifecycle_hook_specification_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerNames", &self.load_balancer_names)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", &self.max_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsCollection", &self.metrics_collection)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", &self.min_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfigurations", &self.notification_configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementGroup", &self.placement_group)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupARNs", &self.target_group_ar_ns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminationPolicies", &self.termination_policies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCZoneIdentifier", &self.vpc_zone_identifier)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AutoScalingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AutoScalingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AutoScalingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name = None;
                let mut availability_zones = None;
                let mut cooldown = None;
                let mut desired_capacity = None;
                let mut health_check_grace_period = None;
                let mut health_check_type = None;
                let mut instance_id = None;
                let mut launch_configuration_name = None;
                let mut lifecycle_hook_specification_list = None;
                let mut load_balancer_names = None;
                let mut max_size = None;
                let mut metrics_collection = None;
                let mut min_size = None;
                let mut notification_configurations = None;
                let mut placement_group = None;
                let mut tags = None;
                let mut target_group_ar_ns = None;
                let mut termination_policies = None;
                let mut vpc_zone_identifier = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZones" => {
                            availability_zones = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Cooldown" => {
                            cooldown = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DesiredCapacity" => {
                            desired_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HealthCheckGracePeriod" => {
                            health_check_grace_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HealthCheckType" => {
                            health_check_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LaunchConfigurationName" => {
                            launch_configuration_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LifecycleHookSpecificationList" => {
                            lifecycle_hook_specification_list = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoadBalancerNames" => {
                            load_balancer_names = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaxSize" => {
                            max_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MetricsCollection" => {
                            metrics_collection = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MinSize" => {
                            min_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NotificationConfigurations" => {
                            notification_configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlacementGroup" => {
                            placement_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetGroupARNs" => {
                            target_group_ar_ns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TerminationPolicies" => {
                            termination_policies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VPCZoneIdentifier" => {
                            vpc_zone_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AutoScalingGroupProperties {
                    auto_scaling_group_name: auto_scaling_group_name,
                    availability_zones: availability_zones,
                    cooldown: cooldown,
                    desired_capacity: desired_capacity,
                    health_check_grace_period: health_check_grace_period,
                    health_check_type: health_check_type,
                    instance_id: instance_id,
                    launch_configuration_name: launch_configuration_name,
                    lifecycle_hook_specification_list: lifecycle_hook_specification_list,
                    load_balancer_names: load_balancer_names,
                    max_size: max_size.ok_or(::serde::de::Error::missing_field("MaxSize"))?,
                    metrics_collection: metrics_collection,
                    min_size: min_size.ok_or(::serde::de::Error::missing_field("MinSize"))?,
                    notification_configurations: notification_configurations,
                    placement_group: placement_group,
                    tags: tags,
                    target_group_ar_ns: target_group_ar_ns,
                    termination_policies: termination_policies,
                    vpc_zone_identifier: vpc_zone_identifier,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AutoScalingGroup {
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
#[derive(Debug)]
pub struct LaunchConfigurationProperties {
    /// Property `AssociatePublicIpAddress`.
    pub associate_public_ip_address: Option<::Value<bool>>,
    /// Property `BlockDeviceMappings`.
    pub block_device_mappings: Option<::ValueList<self::launch_configuration::BlockDeviceMapping>>,
    /// Property `ClassicLinkVPCId`.
    pub classic_link_vpc_id: Option<::Value<String>>,
    /// Property `ClassicLinkVPCSecurityGroups`.
    pub classic_link_vpc_security_groups: Option<::ValueList<String>>,
    /// Property `EbsOptimized`.
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property `IamInstanceProfile`.
    pub iam_instance_profile: Option<::Value<String>>,
    /// Property `ImageId`.
    pub image_id: ::Value<String>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
    /// Property `InstanceMonitoring`.
    pub instance_monitoring: Option<::Value<bool>>,
    /// Property `InstanceType`.
    pub instance_type: ::Value<String>,
    /// Property `KernelId`.
    pub kernel_id: Option<::Value<String>>,
    /// Property `KeyName`.
    pub key_name: Option<::Value<String>>,
    /// Property `PlacementTenancy`.
    pub placement_tenancy: Option<::Value<String>>,
    /// Property `RamDiskId`.
    pub ram_disk_id: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    pub security_groups: Option<::ValueList<String>>,
    /// Property `SpotPrice`.
    pub spot_price: Option<::Value<String>>,
    /// Property `UserData`.
    pub user_data: Option<::Value<String>>,
}

impl ::serde::Serialize for LaunchConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatePublicIpAddress", &self.associate_public_ip_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", &self.block_device_mappings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClassicLinkVPCId", &self.classic_link_vpc_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClassicLinkVPCSecurityGroups", &self.classic_link_vpc_security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamInstanceProfile", &self.iam_instance_profile)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageId", &self.image_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceMonitoring", &self.instance_monitoring)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelId", &self.kernel_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyName", &self.key_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementTenancy", &self.placement_tenancy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RamDiskId", &self.ram_disk_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotPrice", &self.spot_price)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserData", &self.user_data)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associate_public_ip_address = None;
                let mut block_device_mappings = None;
                let mut classic_link_vpc_id = None;
                let mut classic_link_vpc_security_groups = None;
                let mut ebs_optimized = None;
                let mut iam_instance_profile = None;
                let mut image_id = None;
                let mut instance_id = None;
                let mut instance_monitoring = None;
                let mut instance_type = None;
                let mut kernel_id = None;
                let mut key_name = None;
                let mut placement_tenancy = None;
                let mut ram_disk_id = None;
                let mut security_groups = None;
                let mut spot_price = None;
                let mut user_data = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociatePublicIpAddress" => {
                            associate_public_ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BlockDeviceMappings" => {
                            block_device_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClassicLinkVPCId" => {
                            classic_link_vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClassicLinkVPCSecurityGroups" => {
                            classic_link_vpc_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EbsOptimized" => {
                            ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IamInstanceProfile" => {
                            iam_instance_profile = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ImageId" => {
                            image_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceMonitoring" => {
                            instance_monitoring = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceType" => {
                            instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KernelId" => {
                            kernel_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KeyName" => {
                            key_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlacementTenancy" => {
                            placement_tenancy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RamDiskId" => {
                            ram_disk_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroups" => {
                            security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SpotPrice" => {
                            spot_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserData" => {
                            user_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(LaunchConfigurationProperties {
                    associate_public_ip_address: associate_public_ip_address,
                    block_device_mappings: block_device_mappings,
                    classic_link_vpc_id: classic_link_vpc_id,
                    classic_link_vpc_security_groups: classic_link_vpc_security_groups,
                    ebs_optimized: ebs_optimized,
                    iam_instance_profile: iam_instance_profile,
                    image_id: image_id.ok_or(::serde::de::Error::missing_field("ImageId"))?,
                    instance_id: instance_id,
                    instance_monitoring: instance_monitoring,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    kernel_id: kernel_id,
                    key_name: key_name,
                    placement_tenancy: placement_tenancy,
                    ram_disk_id: ram_disk_id,
                    security_groups: security_groups,
                    spot_price: spot_price,
                    user_data: user_data,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LaunchConfiguration {
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
#[derive(Debug)]
pub struct LifecycleHookProperties {
    /// Property `AutoScalingGroupName`.
    pub auto_scaling_group_name: ::Value<String>,
    /// Property `DefaultResult`.
    pub default_result: Option<::Value<String>>,
    /// Property `HeartbeatTimeout`.
    pub heartbeat_timeout: Option<::Value<u32>>,
    /// Property `LifecycleHookName`.
    pub lifecycle_hook_name: Option<::Value<String>>,
    /// Property `LifecycleTransition`.
    pub lifecycle_transition: ::Value<String>,
    /// Property `NotificationMetadata`.
    pub notification_metadata: Option<::Value<String>>,
    /// Property `NotificationTargetARN`.
    pub notification_target_arn: Option<::Value<String>>,
    /// Property `RoleARN`.
    pub role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for LifecycleHookProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResult", &self.default_result)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatTimeout", &self.heartbeat_timeout)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleHookName", &self.lifecycle_hook_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleTransition", &self.lifecycle_transition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationMetadata", &self.notification_metadata)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTargetARN", &self.notification_target_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LifecycleHookProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleHookProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LifecycleHookProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LifecycleHookProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name = None;
                let mut default_result = None;
                let mut heartbeat_timeout = None;
                let mut lifecycle_hook_name = None;
                let mut lifecycle_transition = None;
                let mut notification_metadata = None;
                let mut notification_target_arn = None;
                let mut role_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultResult" => {
                            default_result = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HeartbeatTimeout" => {
                            heartbeat_timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LifecycleHookName" => {
                            lifecycle_hook_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LifecycleTransition" => {
                            lifecycle_transition = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NotificationMetadata" => {
                            notification_metadata = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NotificationTargetARN" => {
                            notification_target_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoleARN" => {
                            role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(LifecycleHookProperties {
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    default_result: default_result,
                    heartbeat_timeout: heartbeat_timeout,
                    lifecycle_hook_name: lifecycle_hook_name,
                    lifecycle_transition: lifecycle_transition.ok_or(::serde::de::Error::missing_field("LifecycleTransition"))?,
                    notification_metadata: notification_metadata,
                    notification_target_arn: notification_target_arn,
                    role_arn: role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LifecycleHook {
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
#[derive(Debug)]
pub struct ScalingPolicyProperties {
    /// Property `AdjustmentType`.
    pub adjustment_type: Option<::Value<String>>,
    /// Property `AutoScalingGroupName`.
    pub auto_scaling_group_name: ::Value<String>,
    /// Property `Cooldown`.
    pub cooldown: Option<::Value<String>>,
    /// Property `EstimatedInstanceWarmup`.
    pub estimated_instance_warmup: Option<::Value<u32>>,
    /// Property `MetricAggregationType`.
    pub metric_aggregation_type: Option<::Value<String>>,
    /// Property `MinAdjustmentMagnitude`.
    pub min_adjustment_magnitude: Option<::Value<u32>>,
    /// Property `PolicyType`.
    pub policy_type: Option<::Value<String>>,
    /// Property `ScalingAdjustment`.
    pub scaling_adjustment: Option<::Value<u32>>,
    /// Property `StepAdjustments`.
    pub step_adjustments: Option<::ValueList<self::scaling_policy::StepAdjustment>>,
    /// Property `TargetTrackingConfiguration`.
    pub target_tracking_configuration: Option<::Value<self::scaling_policy::TargetTrackingConfiguration>>,
}

impl ::serde::Serialize for ScalingPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", &self.adjustment_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cooldown", &self.cooldown)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EstimatedInstanceWarmup", &self.estimated_instance_warmup)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricAggregationType", &self.metric_aggregation_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinAdjustmentMagnitude", &self.min_adjustment_magnitude)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", &self.policy_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", &self.scaling_adjustment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepAdjustments", &self.step_adjustments)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTrackingConfiguration", &self.target_tracking_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScalingPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScalingPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScalingPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut adjustment_type = None;
                let mut auto_scaling_group_name = None;
                let mut cooldown = None;
                let mut estimated_instance_warmup = None;
                let mut metric_aggregation_type = None;
                let mut min_adjustment_magnitude = None;
                let mut policy_type = None;
                let mut scaling_adjustment = None;
                let mut step_adjustments = None;
                let mut target_tracking_configuration = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdjustmentType" => {
                            adjustment_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Cooldown" => {
                            cooldown = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EstimatedInstanceWarmup" => {
                            estimated_instance_warmup = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MetricAggregationType" => {
                            metric_aggregation_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MinAdjustmentMagnitude" => {
                            min_adjustment_magnitude = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PolicyType" => {
                            policy_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ScalingAdjustment" => {
                            scaling_adjustment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StepAdjustments" => {
                            step_adjustments = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetTrackingConfiguration" => {
                            target_tracking_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ScalingPolicyProperties {
                    adjustment_type: adjustment_type,
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    cooldown: cooldown,
                    estimated_instance_warmup: estimated_instance_warmup,
                    metric_aggregation_type: metric_aggregation_type,
                    min_adjustment_magnitude: min_adjustment_magnitude,
                    policy_type: policy_type,
                    scaling_adjustment: scaling_adjustment,
                    step_adjustments: step_adjustments,
                    target_tracking_configuration: target_tracking_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScalingPolicy {
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
#[derive(Debug)]
pub struct ScheduledActionProperties {
    /// Property `AutoScalingGroupName`.
    pub auto_scaling_group_name: ::Value<String>,
    /// Property `DesiredCapacity`.
    pub desired_capacity: Option<::Value<u32>>,
    /// Property `EndTime`.
    pub end_time: Option<::Value<String>>,
    /// Property `MaxSize`.
    pub max_size: Option<::Value<u32>>,
    /// Property `MinSize`.
    pub min_size: Option<::Value<u32>>,
    /// Property `Recurrence`.
    pub recurrence: Option<::Value<String>>,
    /// Property `StartTime`.
    pub start_time: Option<::Value<String>>,
}

impl ::serde::Serialize for ScheduledActionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupName", &self.auto_scaling_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredCapacity", &self.desired_capacity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", &self.end_time)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSize", &self.max_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSize", &self.min_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recurrence", &self.recurrence)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduledActionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledActionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduledActionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduledActionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_name = None;
                let mut desired_capacity = None;
                let mut end_time = None;
                let mut max_size = None;
                let mut min_size = None;
                let mut recurrence = None;
                let mut start_time = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupName" => {
                            auto_scaling_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DesiredCapacity" => {
                            desired_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EndTime" => {
                            end_time = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaxSize" => {
                            max_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MinSize" => {
                            min_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Recurrence" => {
                            recurrence = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StartTime" => {
                            start_time = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ScheduledActionProperties {
                    auto_scaling_group_name: auto_scaling_group_name.ok_or(::serde::de::Error::missing_field("AutoScalingGroupName"))?,
                    desired_capacity: desired_capacity,
                    end_time: end_time,
                    max_size: max_size,
                    min_size: min_size,
                    recurrence: recurrence,
                    start_time: start_time,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScheduledAction {
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
    #[derive(Debug)]
    pub struct LifecycleHookSpecification {
        /// Property `DefaultResult`.
        pub default_result: Option<::Value<String>>,
        /// Property `HeartbeatTimeout`.
        pub heartbeat_timeout: Option<::Value<u32>>,
        /// Property `LifecycleHookName`.
        pub lifecycle_hook_name: ::Value<String>,
        /// Property `LifecycleTransition`.
        pub lifecycle_transition: ::Value<String>,
        /// Property `NotificationMetadata`.
        pub notification_metadata: Option<::Value<String>>,
        /// Property `NotificationTargetARN`.
        pub notification_target_arn: Option<::Value<String>>,
        /// Property `RoleARN`.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LifecycleHookSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResult", &self.default_result)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatTimeout", &self.heartbeat_timeout)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleHookName", &self.lifecycle_hook_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleTransition", &self.lifecycle_transition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationMetadata", &self.notification_metadata)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTargetARN", &self.notification_target_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LifecycleHookSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleHookSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecycleHookSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecycleHookSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_result = None;
                    let mut heartbeat_timeout = None;
                    let mut lifecycle_hook_name = None;
                    let mut lifecycle_transition = None;
                    let mut notification_metadata = None;
                    let mut notification_target_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultResult" => {
                                default_result = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HeartbeatTimeout" => {
                                heartbeat_timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LifecycleHookName" => {
                                lifecycle_hook_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LifecycleTransition" => {
                                lifecycle_transition = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NotificationMetadata" => {
                                notification_metadata = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NotificationTargetARN" => {
                                notification_target_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecycleHookSpecification {
                        default_result: default_result,
                        heartbeat_timeout: heartbeat_timeout,
                        lifecycle_hook_name: lifecycle_hook_name.ok_or(::serde::de::Error::missing_field("LifecycleHookName"))?,
                        lifecycle_transition: lifecycle_transition.ok_or(::serde::de::Error::missing_field("LifecycleTransition"))?,
                        notification_metadata: notification_metadata,
                        notification_target_arn: notification_target_arn,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.MetricsCollection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-metricscollection.html) property type.
    #[derive(Debug)]
    pub struct MetricsCollection {
        /// Property `Granularity`.
        pub granularity: ::Value<String>,
        /// Property `Metrics`.
        pub metrics: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for MetricsCollection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Granularity", &self.granularity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metrics", &self.metrics)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricsCollection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricsCollection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricsCollection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricsCollection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut granularity = None;
                    let mut metrics = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Granularity" => {
                                granularity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Metrics" => {
                                metrics = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricsCollection {
                        granularity: granularity.ok_or(::serde::de::Error::missing_field("Granularity"))?,
                        metrics: metrics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-notificationconfigurations.html) property type.
    #[derive(Debug)]
    pub struct NotificationConfiguration {
        /// Property `NotificationTypes`.
        pub notification_types: Option<::ValueList<String>>,
        /// Property `TopicARN`.
        pub topic_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for NotificationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTypes", &self.notification_types)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicARN", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notification_types = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotificationTypes" => {
                                notification_types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicARN" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationConfiguration {
                        notification_types: notification_types,
                        topic_arn: topic_arn.ok_or(::serde::de::Error::missing_field("TopicARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::AutoScalingGroup.TagProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-tags.html) property type.
    #[derive(Debug)]
    pub struct TagProperty {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `PropagateAtLaunch`.
        pub propagate_at_launch: ::Value<bool>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TagProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateAtLaunch", &self.propagate_at_launch)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut propagate_at_launch = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PropagateAtLaunch" => {
                                propagate_at_launch = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TagProperty {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        propagate_at_launch: propagate_at_launch.ok_or(::serde::de::Error::missing_field("PropagateAtLaunch"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod launch_configuration {
    //! Property types for the `LaunchConfiguration` resource.

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-template.html) property type.
    #[derive(Debug)]
    pub struct BlockDevice {
        /// Property `DeleteOnTermination`.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Encrypted`.
        pub encrypted: Option<::Value<bool>>,
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlockDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", &self.snapshot_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", &self.volume_size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockDevice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDevice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDevice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDevice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination = None;
                    let mut encrypted = None;
                    let mut iops = None;
                    let mut snapshot_id = None;
                    let mut volume_size = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Encrypted" => {
                                encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SnapshotId" => {
                                snapshot_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeSize" => {
                                volume_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDevice {
                        delete_on_termination: delete_on_termination,
                        encrypted: encrypted,
                        iops: iops,
                        snapshot_id: snapshot_id,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::LaunchConfiguration.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-as-launchconfig-blockdev-mapping.html) property type.
    #[derive(Debug)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        pub device_name: ::Value<String>,
        /// Property `Ebs`.
        pub ebs: Option<::Value<BlockDevice>>,
        /// Property `NoDevice`.
        pub no_device: Option<::Value<bool>>,
        /// Property `VirtualName`.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", &self.device_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", &self.ebs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", &self.no_device)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", &self.virtual_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name = None;
                    let mut ebs = None;
                    let mut no_device = None;
                    let mut virtual_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ebs" => {
                                ebs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NoDevice" => {
                                no_device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VirtualName" => {
                                virtual_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDeviceMapping {
                        device_name: device_name.ok_or(::serde::de::Error::missing_field("DeviceName"))?,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::AutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html) property type.
    #[derive(Debug)]
    pub struct CustomizedMetricSpecification {
        /// Property `Dimensions`.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `MetricName`.
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        pub namespace: ::Value<String>,
        /// Property `Statistic`.
        pub statistic: ::Value<String>,
        /// Property `Unit`.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomizedMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomizedMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomizedMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomizedMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomizedMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions = None;
                    let mut metric_name = None;
                    let mut namespace = None;
                    let mut statistic = None;
                    let mut unit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricName" => {
                                metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Namespace" => {
                                namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Statistic" => {
                                statistic = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Unit" => {
                                unit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomizedMetricSpecification {
                        dimensions: dimensions,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                        statistic: statistic.ok_or(::serde::de::Error::missing_field("Statistic"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html) property type.
    #[derive(Debug)]
    pub struct MetricDimension {
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
    #[derive(Debug)]
    pub struct PredefinedMetricSpecification {
        /// Property `PredefinedMetricType`.
        pub predefined_metric_type: ::Value<String>,
        /// Property `ResourceLabel`.
        pub resource_label: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PredefinedMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedMetricType", &self.predefined_metric_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLabel", &self.resource_label)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PredefinedMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PredefinedMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PredefinedMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PredefinedMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut predefined_metric_type = None;
                    let mut resource_label = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PredefinedMetricType" => {
                                predefined_metric_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourceLabel" => {
                                resource_label = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PredefinedMetricSpecification {
                        predefined_metric_type: predefined_metric_type.ok_or(::serde::de::Error::missing_field("PredefinedMetricType"))?,
                        resource_label: resource_label,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustments.html) property type.
    #[derive(Debug)]
    pub struct StepAdjustment {
        /// Property `MetricIntervalLowerBound`.
        pub metric_interval_lower_bound: Option<::Value<f64>>,
        /// Property `MetricIntervalUpperBound`.
        pub metric_interval_upper_bound: Option<::Value<f64>>,
        /// Property `ScalingAdjustment`.
        pub scaling_adjustment: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StepAdjustment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricIntervalLowerBound", &self.metric_interval_lower_bound)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricIntervalUpperBound", &self.metric_interval_upper_bound)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", &self.scaling_adjustment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StepAdjustment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StepAdjustment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StepAdjustment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StepAdjustment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_interval_lower_bound = None;
                    let mut metric_interval_upper_bound = None;
                    let mut scaling_adjustment = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricIntervalLowerBound" => {
                                metric_interval_lower_bound = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricIntervalUpperBound" => {
                                metric_interval_upper_bound = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StepAdjustment {
                        metric_interval_lower_bound: metric_interval_lower_bound,
                        metric_interval_upper_bound: metric_interval_upper_bound,
                        scaling_adjustment: scaling_adjustment.ok_or(::serde::de::Error::missing_field("ScalingAdjustment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScaling::ScalingPolicy.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html) property type.
    #[derive(Debug)]
    pub struct TargetTrackingConfiguration {
        /// Property `CustomizedMetricSpecification`.
        pub customized_metric_specification: Option<::Value<CustomizedMetricSpecification>>,
        /// Property `DisableScaleIn`.
        pub disable_scale_in: Option<::Value<bool>>,
        /// Property `PredefinedMetricSpecification`.
        pub predefined_metric_specification: Option<::Value<PredefinedMetricSpecification>>,
        /// Property `TargetValue`.
        pub target_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TargetTrackingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomizedMetricSpecification", &self.customized_metric_specification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableScaleIn", &self.disable_scale_in)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedMetricSpecification", &self.predefined_metric_specification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetValue", &self.target_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetTrackingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetTrackingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetTrackingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetTrackingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customized_metric_specification = None;
                    let mut disable_scale_in = None;
                    let mut predefined_metric_specification = None;
                    let mut target_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomizedMetricSpecification" => {
                                customized_metric_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DisableScaleIn" => {
                                disable_scale_in = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PredefinedMetricSpecification" => {
                                predefined_metric_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetValue" => {
                                target_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetTrackingConfiguration {
                        customized_metric_specification: customized_metric_specification,
                        disable_scale_in: disable_scale_in,
                        predefined_metric_specification: predefined_metric_specification,
                        target_value: target_value.ok_or(::serde::de::Error::missing_field("TargetValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
