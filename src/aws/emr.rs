//! Types for the `EMR` service.

/// The [`AWS::EMR::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug)]
pub struct ClusterProperties {
    /// Property `AdditionalInfo`.
    pub additional_info: Option<::Value<::json::Value>>,
    /// Property `Applications`.
    pub applications: Option<::ValueList<self::cluster::Application>>,
    /// Property `AutoScalingRole`.
    pub auto_scaling_role: Option<::Value<String>>,
    /// Property `BootstrapActions`.
    pub bootstrap_actions: Option<::ValueList<self::cluster::BootstrapActionConfig>>,
    /// Property `Configurations`.
    pub configurations: Option<::ValueList<self::cluster::Configuration>>,
    /// Property `CustomAmiId`.
    pub custom_ami_id: Option<::Value<String>>,
    /// Property `EbsRootVolumeSize`.
    pub ebs_root_volume_size: Option<::Value<u32>>,
    /// Property `Instances`.
    pub instances: ::Value<self::cluster::JobFlowInstancesConfig>,
    /// Property `JobFlowRole`.
    pub job_flow_role: ::Value<String>,
    /// Property `LogUri`.
    pub log_uri: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `ReleaseLabel`.
    pub release_label: Option<::Value<String>>,
    /// Property `ScaleDownBehavior`.
    pub scale_down_behavior: Option<::Value<String>>,
    /// Property `SecurityConfiguration`.
    pub security_configuration: Option<::Value<String>>,
    /// Property `ServiceRole`.
    pub service_role: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VisibleToAllUsers`.
    pub visible_to_all_users: Option<::Value<bool>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInfo", &self.additional_info)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Applications", &self.applications)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingRole", &self.auto_scaling_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BootstrapActions", &self.bootstrap_actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomAmiId", &self.custom_ami_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsRootVolumeSize", &self.ebs_root_volume_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instances", &self.instances)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobFlowRole", &self.job_flow_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogUri", &self.log_uri)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReleaseLabel", &self.release_label)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleDownBehavior", &self.scale_down_behavior)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityConfiguration", &self.security_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibleToAllUsers", &self.visible_to_all_users)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_info = None;
                let mut applications = None;
                let mut auto_scaling_role = None;
                let mut bootstrap_actions = None;
                let mut configurations = None;
                let mut custom_ami_id = None;
                let mut ebs_root_volume_size = None;
                let mut instances = None;
                let mut job_flow_role = None;
                let mut log_uri = None;
                let mut name = None;
                let mut release_label = None;
                let mut scale_down_behavior = None;
                let mut security_configuration = None;
                let mut service_role = None;
                let mut tags = None;
                let mut visible_to_all_users = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalInfo" => {
                            additional_info = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Applications" => {
                            applications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoScalingRole" => {
                            auto_scaling_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BootstrapActions" => {
                            bootstrap_actions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Configurations" => {
                            configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomAmiId" => {
                            custom_ami_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EbsRootVolumeSize" => {
                            ebs_root_volume_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Instances" => {
                            instances = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "JobFlowRole" => {
                            job_flow_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LogUri" => {
                            log_uri = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReleaseLabel" => {
                            release_label = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ScaleDownBehavior" => {
                            scale_down_behavior = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityConfiguration" => {
                            security_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceRole" => {
                            service_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VisibleToAllUsers" => {
                            visible_to_all_users = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    additional_info: additional_info,
                    applications: applications,
                    auto_scaling_role: auto_scaling_role,
                    bootstrap_actions: bootstrap_actions,
                    configurations: configurations,
                    custom_ami_id: custom_ami_id,
                    ebs_root_volume_size: ebs_root_volume_size,
                    instances: instances.ok_or(::serde::de::Error::missing_field("Instances"))?,
                    job_flow_role: job_flow_role.ok_or(::serde::de::Error::missing_field("JobFlowRole"))?,
                    log_uri: log_uri,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    release_label: release_label,
                    scale_down_behavior: scale_down_behavior,
                    security_configuration: security_configuration,
                    service_role: service_role.ok_or(::serde::de::Error::missing_field("ServiceRole"))?,
                    tags: tags,
                    visible_to_all_users: visible_to_all_users,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::EMR::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::EMR::InstanceFleetConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html) resource type.
#[derive(Debug)]
pub struct InstanceFleetConfig {
    properties: InstanceFleetConfigProperties
}

/// Properties for the `InstanceFleetConfig` resource.
#[derive(Debug)]
pub struct InstanceFleetConfigProperties {
    /// Property `ClusterId`.
    pub cluster_id: ::Value<String>,
    /// Property `InstanceFleetType`.
    pub instance_fleet_type: ::Value<String>,
    /// Property `InstanceTypeConfigs`.
    pub instance_type_configs: Option<::ValueList<self::instance_fleet_config::InstanceTypeConfig>>,
    /// Property `LaunchSpecifications`.
    pub launch_specifications: Option<::Value<self::instance_fleet_config::InstanceFleetProvisioningSpecifications>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `TargetOnDemandCapacity`.
    pub target_on_demand_capacity: Option<::Value<u32>>,
    /// Property `TargetSpotCapacity`.
    pub target_spot_capacity: Option<::Value<u32>>,
}

impl ::serde::Serialize for InstanceFleetConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterId", &self.cluster_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceFleetType", &self.instance_fleet_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypeConfigs", &self.instance_type_configs)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchSpecifications", &self.launch_specifications)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOnDemandCapacity", &self.target_on_demand_capacity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetSpotCapacity", &self.target_spot_capacity)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceFleetConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceFleetConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceFleetConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceFleetConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_id = None;
                let mut instance_fleet_type = None;
                let mut instance_type_configs = None;
                let mut launch_specifications = None;
                let mut name = None;
                let mut target_on_demand_capacity = None;
                let mut target_spot_capacity = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterId" => {
                            cluster_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceFleetType" => {
                            instance_fleet_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceTypeConfigs" => {
                            instance_type_configs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LaunchSpecifications" => {
                            launch_specifications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetOnDemandCapacity" => {
                            target_on_demand_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetSpotCapacity" => {
                            target_spot_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(InstanceFleetConfigProperties {
                    cluster_id: cluster_id.ok_or(::serde::de::Error::missing_field("ClusterId"))?,
                    instance_fleet_type: instance_fleet_type.ok_or(::serde::de::Error::missing_field("InstanceFleetType"))?,
                    instance_type_configs: instance_type_configs,
                    launch_specifications: launch_specifications,
                    name: name,
                    target_on_demand_capacity: target_on_demand_capacity,
                    target_spot_capacity: target_spot_capacity,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceFleetConfig {
    type Properties = InstanceFleetConfigProperties;
    const TYPE: &'static str = "AWS::EMR::InstanceFleetConfig";
    fn properties(&self) -> &InstanceFleetConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceFleetConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceFleetConfig {}

impl From<InstanceFleetConfigProperties> for InstanceFleetConfig {
    fn from(properties: InstanceFleetConfigProperties) -> InstanceFleetConfig {
        InstanceFleetConfig { properties }
    }
}

/// The [`AWS::EMR::InstanceGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html) resource type.
#[derive(Debug)]
pub struct InstanceGroupConfig {
    properties: InstanceGroupConfigProperties
}

/// Properties for the `InstanceGroupConfig` resource.
#[derive(Debug)]
pub struct InstanceGroupConfigProperties {
    /// Property `AutoScalingPolicy`.
    pub auto_scaling_policy: Option<::Value<self::instance_group_config::AutoScalingPolicy>>,
    /// Property `BidPrice`.
    pub bid_price: Option<::Value<String>>,
    /// Property `Configurations`.
    pub configurations: Option<::ValueList<self::instance_group_config::Configuration>>,
    /// Property `EbsConfiguration`.
    pub ebs_configuration: Option<::Value<self::instance_group_config::EbsConfiguration>>,
    /// Property `InstanceCount`.
    pub instance_count: ::Value<u32>,
    /// Property `InstanceRole`.
    pub instance_role: ::Value<String>,
    /// Property `InstanceType`.
    pub instance_type: ::Value<String>,
    /// Property `JobFlowId`.
    pub job_flow_id: ::Value<String>,
    /// Property `Market`.
    pub market: Option<::Value<String>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for InstanceGroupConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingPolicy", &self.auto_scaling_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", &self.bid_price)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", &self.ebs_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceRole", &self.instance_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobFlowId", &self.job_flow_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", &self.market)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceGroupConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceGroupConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceGroupConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceGroupConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_policy = None;
                let mut bid_price = None;
                let mut configurations = None;
                let mut ebs_configuration = None;
                let mut instance_count = None;
                let mut instance_role = None;
                let mut instance_type = None;
                let mut job_flow_id = None;
                let mut market = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingPolicy" => {
                            auto_scaling_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BidPrice" => {
                            bid_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Configurations" => {
                            configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EbsConfiguration" => {
                            ebs_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceCount" => {
                            instance_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceRole" => {
                            instance_role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceType" => {
                            instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "JobFlowId" => {
                            job_flow_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Market" => {
                            market = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(InstanceGroupConfigProperties {
                    auto_scaling_policy: auto_scaling_policy,
                    bid_price: bid_price,
                    configurations: configurations,
                    ebs_configuration: ebs_configuration,
                    instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                    instance_role: instance_role.ok_or(::serde::de::Error::missing_field("InstanceRole"))?,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    job_flow_id: job_flow_id.ok_or(::serde::de::Error::missing_field("JobFlowId"))?,
                    market: market,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceGroupConfig {
    type Properties = InstanceGroupConfigProperties;
    const TYPE: &'static str = "AWS::EMR::InstanceGroupConfig";
    fn properties(&self) -> &InstanceGroupConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceGroupConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceGroupConfig {}

impl From<InstanceGroupConfigProperties> for InstanceGroupConfig {
    fn from(properties: InstanceGroupConfigProperties) -> InstanceGroupConfig {
        InstanceGroupConfig { properties }
    }
}

/// The [`AWS::EMR::SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-securityconfiguration.html) resource type.
#[derive(Debug)]
pub struct SecurityConfiguration {
    properties: SecurityConfigurationProperties
}

/// Properties for the `SecurityConfiguration` resource.
#[derive(Debug)]
pub struct SecurityConfigurationProperties {
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `SecurityConfiguration`.
    pub security_configuration: ::Value<::json::Value>,
}

impl ::serde::Serialize for SecurityConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityConfiguration", &self.security_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name = None;
                let mut security_configuration = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityConfiguration" => {
                            security_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SecurityConfigurationProperties {
                    name: name,
                    security_configuration: security_configuration.ok_or(::serde::de::Error::missing_field("SecurityConfiguration"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityConfiguration {
    type Properties = SecurityConfigurationProperties;
    const TYPE: &'static str = "AWS::EMR::SecurityConfiguration";
    fn properties(&self) -> &SecurityConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityConfiguration {}

impl From<SecurityConfigurationProperties> for SecurityConfiguration {
    fn from(properties: SecurityConfigurationProperties) -> SecurityConfiguration {
        SecurityConfiguration { properties }
    }
}

/// The [`AWS::EMR::Step`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html) resource type.
#[derive(Debug)]
pub struct Step {
    properties: StepProperties
}

/// Properties for the `Step` resource.
#[derive(Debug)]
pub struct StepProperties {
    /// Property `ActionOnFailure`.
    pub action_on_failure: ::Value<String>,
    /// Property `HadoopJarStep`.
    pub hadoop_jar_step: ::Value<self::step::HadoopJarStepConfig>,
    /// Property `JobFlowId`.
    pub job_flow_id: ::Value<String>,
    /// Property `Name`.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for StepProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionOnFailure", &self.action_on_failure)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HadoopJarStep", &self.hadoop_jar_step)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobFlowId", &self.job_flow_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StepProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StepProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StepProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StepProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action_on_failure = None;
                let mut hadoop_jar_step = None;
                let mut job_flow_id = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionOnFailure" => {
                            action_on_failure = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HadoopJarStep" => {
                            hadoop_jar_step = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "JobFlowId" => {
                            job_flow_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(StepProperties {
                    action_on_failure: action_on_failure.ok_or(::serde::de::Error::missing_field("ActionOnFailure"))?,
                    hadoop_jar_step: hadoop_jar_step.ok_or(::serde::de::Error::missing_field("HadoopJarStep"))?,
                    job_flow_id: job_flow_id.ok_or(::serde::de::Error::missing_field("JobFlowId"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Step {
    type Properties = StepProperties;
    const TYPE: &'static str = "AWS::EMR::Step";
    fn properties(&self) -> &StepProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StepProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Step {}

impl From<StepProperties> for Step {
    fn from(properties: StepProperties) -> Step {
        Step { properties }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::EMR::Cluster.Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-application.html) property type.
    #[derive(Debug)]
    pub struct Application {
        /// Property `AdditionalInfo`.
        pub additional_info: Option<::ValueMap<String>>,
        /// Property `Args`.
        pub args: Option<::ValueList<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Version`.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Application {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInfo", &self.additional_info)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Args", &self.args)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Application {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Application, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Application;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Application")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut additional_info = None;
                    let mut args = None;
                    let mut name = None;
                    let mut version = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalInfo" => {
                                additional_info = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Args" => {
                                args = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Version" => {
                                version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Application {
                        additional_info: additional_info,
                        args: args,
                        name: name,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoscalingpolicy.html) property type.
    #[derive(Debug)]
    pub struct AutoScalingPolicy {
        /// Property `Constraints`.
        pub constraints: ::Value<ScalingConstraints>,
        /// Property `Rules`.
        pub rules: ::ValueList<ScalingRule>,
    }

    impl ::codec::SerializeValue for AutoScalingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", &self.constraints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoScalingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoScalingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoScalingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints = None;
                    let mut rules = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Rules" => {
                                rules = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoScalingPolicy {
                        constraints: constraints.ok_or(::serde::de::Error::missing_field("Constraints"))?,
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.BootstrapActionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-bootstrapactionconfig.html) property type.
    #[derive(Debug)]
    pub struct BootstrapActionConfig {
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `ScriptBootstrapAction`.
        pub script_bootstrap_action: ::Value<ScriptBootstrapActionConfig>,
    }

    impl ::codec::SerializeValue for BootstrapActionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScriptBootstrapAction", &self.script_bootstrap_action)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BootstrapActionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BootstrapActionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BootstrapActionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BootstrapActionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut script_bootstrap_action = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ScriptBootstrapAction" => {
                                script_bootstrap_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BootstrapActionConfig {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        script_bootstrap_action: script_bootstrap_action.ok_or(::serde::de::Error::missing_field("ScriptBootstrapAction"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html) property type.
    #[derive(Debug)]
    pub struct CloudWatchAlarmDefinition {
        /// Property `ComparisonOperator`.
        pub comparison_operator: ::Value<String>,
        /// Property `Dimensions`.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `EvaluationPeriods`.
        pub evaluation_periods: Option<::Value<u32>>,
        /// Property `MetricName`.
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        pub namespace: Option<::Value<String>>,
        /// Property `Period`.
        pub period: ::Value<u32>,
        /// Property `Statistic`.
        pub statistic: Option<::Value<String>>,
        /// Property `Threshold`.
        pub threshold: ::Value<f64>,
        /// Property `Unit`.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchAlarmDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", &self.evaluation_periods)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchAlarmDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchAlarmDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchAlarmDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchAlarmDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator = None;
                    let mut dimensions = None;
                    let mut evaluation_periods = None;
                    let mut metric_name = None;
                    let mut namespace = None;
                    let mut period = None;
                    let mut statistic = None;
                    let mut threshold = None;
                    let mut unit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Dimensions" => {
                                dimensions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EvaluationPeriods" => {
                                evaluation_periods = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricName" => {
                                metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Namespace" => {
                                namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Period" => {
                                period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Statistic" => {
                                statistic = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Threshold" => {
                                threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Unit" => {
                                unit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchAlarmDefinition {
                        comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                        dimensions: dimensions,
                        evaluation_periods: evaluation_periods,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        namespace: namespace,
                        period: period.ok_or(::serde::de::Error::missing_field("Period"))?,
                        statistic: statistic,
                        threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html) property type.
    #[derive(Debug)]
    pub struct Configuration {
        /// Property `Classification`.
        pub classification: Option<::Value<String>>,
        /// Property `ConfigurationProperties`.
        pub configuration_properties: Option<::ValueMap<String>>,
        /// Property `Configurations`.
        pub configurations: Option<::ValueList<Configuration>>,
    }

    impl ::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", &self.classification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", &self.configuration_properties)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Configuration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Configuration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Configuration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Configuration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classification = None;
                    let mut configuration_properties = None;
                    let mut configurations = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ConfigurationProperties" => {
                                configuration_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configurations" => {
                                configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Configuration {
                        classification: classification,
                        configuration_properties: configuration_properties,
                        configurations: configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsblockdeviceconfig.html) property type.
    #[derive(Debug)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property `VolumesPerInstance`.
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EbsBlockDeviceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSpecification", &self.volume_specification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesPerInstance", &self.volumes_per_instance)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsBlockDeviceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsBlockDeviceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsBlockDeviceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsBlockDeviceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut volume_specification = None;
                    let mut volumes_per_instance = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VolumeSpecification" => {
                                volume_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumesPerInstance" => {
                                volumes_per_instance = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsBlockDeviceConfig {
                        volume_specification: volume_specification.ok_or(::serde::de::Error::missing_field("VolumeSpecification"))?,
                        volumes_per_instance: volumes_per_instance,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property `EbsOptimized`.
        pub ebs_optimized: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EbsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsBlockDeviceConfigs", &self.ebs_block_device_configs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ebs_block_device_configs = None;
                    let mut ebs_optimized = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EbsBlockDeviceConfigs" => {
                                ebs_block_device_configs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsOptimized" => {
                                ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsConfiguration {
                        ebs_block_device_configs: ebs_block_device_configs,
                        ebs_optimized: ebs_optimized,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.InstanceFleetConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html) property type.
    #[derive(Debug)]
    pub struct InstanceFleetConfig {
        /// Property `InstanceTypeConfigs`.
        pub instance_type_configs: Option<::ValueList<InstanceTypeConfig>>,
        /// Property `LaunchSpecifications`.
        pub launch_specifications: Option<::Value<InstanceFleetProvisioningSpecifications>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `TargetOnDemandCapacity`.
        pub target_on_demand_capacity: Option<::Value<u32>>,
        /// Property `TargetSpotCapacity`.
        pub target_spot_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InstanceFleetConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypeConfigs", &self.instance_type_configs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchSpecifications", &self.launch_specifications)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOnDemandCapacity", &self.target_on_demand_capacity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetSpotCapacity", &self.target_spot_capacity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceFleetConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceFleetConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceFleetConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceFleetConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_type_configs = None;
                    let mut launch_specifications = None;
                    let mut name = None;
                    let mut target_on_demand_capacity = None;
                    let mut target_spot_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceTypeConfigs" => {
                                instance_type_configs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LaunchSpecifications" => {
                                launch_specifications = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetOnDemandCapacity" => {
                                target_on_demand_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetSpotCapacity" => {
                                target_spot_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceFleetConfig {
                        instance_type_configs: instance_type_configs,
                        launch_specifications: launch_specifications,
                        name: name,
                        target_on_demand_capacity: target_on_demand_capacity,
                        target_spot_capacity: target_spot_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.InstanceFleetProvisioningSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetprovisioningspecifications.html) property type.
    #[derive(Debug)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property `SpotSpecification`.
        pub spot_specification: ::Value<SpotProvisioningSpecification>,
    }

    impl ::codec::SerializeValue for InstanceFleetProvisioningSpecifications {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotSpecification", &self.spot_specification)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceFleetProvisioningSpecifications {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceFleetProvisioningSpecifications, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceFleetProvisioningSpecifications;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceFleetProvisioningSpecifications")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut spot_specification = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpotSpecification" => {
                                spot_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceFleetProvisioningSpecifications {
                        spot_specification: spot_specification.ok_or(::serde::de::Error::missing_field("SpotSpecification"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.InstanceGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html) property type.
    #[derive(Debug)]
    pub struct InstanceGroupConfig {
        /// Property `AutoScalingPolicy`.
        pub auto_scaling_policy: Option<::Value<AutoScalingPolicy>>,
        /// Property `BidPrice`.
        pub bid_price: Option<::Value<String>>,
        /// Property `Configurations`.
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property `EbsConfiguration`.
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property `InstanceCount`.
        pub instance_count: ::Value<u32>,
        /// Property `InstanceType`.
        pub instance_type: ::Value<String>,
        /// Property `Market`.
        pub market: Option<::Value<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceGroupConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingPolicy", &self.auto_scaling_policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", &self.bid_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", &self.ebs_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", &self.market)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceGroupConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceGroupConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceGroupConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceGroupConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_scaling_policy = None;
                    let mut bid_price = None;
                    let mut configurations = None;
                    let mut ebs_configuration = None;
                    let mut instance_count = None;
                    let mut instance_type = None;
                    let mut market = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoScalingPolicy" => {
                                auto_scaling_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BidPrice" => {
                                bid_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configurations" => {
                                configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsConfiguration" => {
                                ebs_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceCount" => {
                                instance_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceType" => {
                                instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Market" => {
                                market = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceGroupConfig {
                        auto_scaling_policy: auto_scaling_policy,
                        bid_price: bid_price,
                        configurations: configurations,
                        ebs_configuration: ebs_configuration,
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        market: market,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.InstanceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html) property type.
    #[derive(Debug)]
    pub struct InstanceTypeConfig {
        /// Property `BidPrice`.
        pub bid_price: Option<::Value<String>>,
        /// Property `BidPriceAsPercentageOfOnDemandPrice`.
        pub bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>>,
        /// Property `Configurations`.
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property `EbsConfiguration`.
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property `InstanceType`.
        pub instance_type: ::Value<String>,
        /// Property `WeightedCapacity`.
        pub weighted_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InstanceTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", &self.bid_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPriceAsPercentageOfOnDemandPrice", &self.bid_price_as_percentage_of_on_demand_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", &self.ebs_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", &self.weighted_capacity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceTypeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceTypeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceTypeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceTypeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bid_price = None;
                    let mut bid_price_as_percentage_of_on_demand_price = None;
                    let mut configurations = None;
                    let mut ebs_configuration = None;
                    let mut instance_type = None;
                    let mut weighted_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BidPrice" => {
                                bid_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BidPriceAsPercentageOfOnDemandPrice" => {
                                bid_price_as_percentage_of_on_demand_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configurations" => {
                                configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsConfiguration" => {
                                ebs_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceType" => {
                                instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceTypeConfig {
                        bid_price: bid_price,
                        bid_price_as_percentage_of_on_demand_price: bid_price_as_percentage_of_on_demand_price,
                        configurations: configurations,
                        ebs_configuration: ebs_configuration,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        weighted_capacity: weighted_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.JobFlowInstancesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html) property type.
    #[derive(Debug)]
    pub struct JobFlowInstancesConfig {
        /// Property `AdditionalMasterSecurityGroups`.
        pub additional_master_security_groups: Option<::ValueList<String>>,
        /// Property `AdditionalSlaveSecurityGroups`.
        pub additional_slave_security_groups: Option<::ValueList<String>>,
        /// Property `CoreInstanceFleet`.
        pub core_instance_fleet: Option<::Value<InstanceFleetConfig>>,
        /// Property `CoreInstanceGroup`.
        pub core_instance_group: Option<::Value<InstanceGroupConfig>>,
        /// Property `Ec2KeyName`.
        pub ec2_key_name: Option<::Value<String>>,
        /// Property `Ec2SubnetId`.
        pub ec2_subnet_id: Option<::Value<String>>,
        /// Property `EmrManagedMasterSecurityGroup`.
        pub emr_managed_master_security_group: Option<::Value<String>>,
        /// Property `EmrManagedSlaveSecurityGroup`.
        pub emr_managed_slave_security_group: Option<::Value<String>>,
        /// Property `HadoopVersion`.
        pub hadoop_version: Option<::Value<String>>,
        /// Property `MasterInstanceFleet`.
        pub master_instance_fleet: Option<::Value<InstanceFleetConfig>>,
        /// Property `MasterInstanceGroup`.
        pub master_instance_group: Option<::Value<InstanceGroupConfig>>,
        /// Property `Placement`.
        pub placement: Option<::Value<PlacementType>>,
        /// Property `ServiceAccessSecurityGroup`.
        pub service_access_security_group: Option<::Value<String>>,
        /// Property `TerminationProtected`.
        pub termination_protected: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for JobFlowInstancesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalMasterSecurityGroups", &self.additional_master_security_groups)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalSlaveSecurityGroups", &self.additional_slave_security_groups)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreInstanceFleet", &self.core_instance_fleet)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreInstanceGroup", &self.core_instance_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2KeyName", &self.ec2_key_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2SubnetId", &self.ec2_subnet_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmrManagedMasterSecurityGroup", &self.emr_managed_master_security_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmrManagedSlaveSecurityGroup", &self.emr_managed_slave_security_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HadoopVersion", &self.hadoop_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterInstanceFleet", &self.master_instance_fleet)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterInstanceGroup", &self.master_instance_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Placement", &self.placement)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessSecurityGroup", &self.service_access_security_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminationProtected", &self.termination_protected)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobFlowInstancesConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobFlowInstancesConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobFlowInstancesConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobFlowInstancesConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut additional_master_security_groups = None;
                    let mut additional_slave_security_groups = None;
                    let mut core_instance_fleet = None;
                    let mut core_instance_group = None;
                    let mut ec2_key_name = None;
                    let mut ec2_subnet_id = None;
                    let mut emr_managed_master_security_group = None;
                    let mut emr_managed_slave_security_group = None;
                    let mut hadoop_version = None;
                    let mut master_instance_fleet = None;
                    let mut master_instance_group = None;
                    let mut placement = None;
                    let mut service_access_security_group = None;
                    let mut termination_protected = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalMasterSecurityGroups" => {
                                additional_master_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "AdditionalSlaveSecurityGroups" => {
                                additional_slave_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CoreInstanceFleet" => {
                                core_instance_fleet = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CoreInstanceGroup" => {
                                core_instance_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ec2KeyName" => {
                                ec2_key_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ec2SubnetId" => {
                                ec2_subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EmrManagedMasterSecurityGroup" => {
                                emr_managed_master_security_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EmrManagedSlaveSecurityGroup" => {
                                emr_managed_slave_security_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HadoopVersion" => {
                                hadoop_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MasterInstanceFleet" => {
                                master_instance_fleet = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MasterInstanceGroup" => {
                                master_instance_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Placement" => {
                                placement = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ServiceAccessSecurityGroup" => {
                                service_access_security_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TerminationProtected" => {
                                termination_protected = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(JobFlowInstancesConfig {
                        additional_master_security_groups: additional_master_security_groups,
                        additional_slave_security_groups: additional_slave_security_groups,
                        core_instance_fleet: core_instance_fleet,
                        core_instance_group: core_instance_group,
                        ec2_key_name: ec2_key_name,
                        ec2_subnet_id: ec2_subnet_id,
                        emr_managed_master_security_group: emr_managed_master_security_group,
                        emr_managed_slave_security_group: emr_managed_slave_security_group,
                        hadoop_version: hadoop_version,
                        master_instance_fleet: master_instance_fleet,
                        master_instance_group: master_instance_group,
                        placement: placement,
                        service_access_security_group: service_access_security_group,
                        termination_protected: termination_protected,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-metricdimension.html) property type.
    #[derive(Debug)]
    pub struct MetricDimension {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
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
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDimension {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.PlacementType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-placementtype.html) property type.
    #[derive(Debug)]
    pub struct PlacementType {
        /// Property `AvailabilityZone`.
        pub availability_zone: ::Value<String>,
    }

    impl ::codec::SerializeValue for PlacementType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementType {
                        availability_zone: availability_zone.ok_or(::serde::de::Error::missing_field("AvailabilityZone"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.ScalingAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingaction.html) property type.
    #[derive(Debug)]
    pub struct ScalingAction {
        /// Property `Market`.
        pub market: Option<::Value<String>>,
        /// Property `SimpleScalingPolicyConfiguration`.
        pub simple_scaling_policy_configuration: ::Value<SimpleScalingPolicyConfiguration>,
    }

    impl ::codec::SerializeValue for ScalingAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", &self.market)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SimpleScalingPolicyConfiguration", &self.simple_scaling_policy_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut market = None;
                    let mut simple_scaling_policy_configuration = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Market" => {
                                market = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SimpleScalingPolicyConfiguration" => {
                                simple_scaling_policy_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingAction {
                        market: market,
                        simple_scaling_policy_configuration: simple_scaling_policy_configuration.ok_or(::serde::de::Error::missing_field("SimpleScalingPolicyConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.ScalingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingconstraints.html) property type.
    #[derive(Debug)]
    pub struct ScalingConstraints {
        /// Property `MaxCapacity`.
        pub max_capacity: ::Value<u32>,
        /// Property `MinCapacity`.
        pub min_capacity: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ScalingConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", &self.max_capacity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", &self.min_capacity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingConstraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingConstraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingConstraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingConstraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_capacity = None;
                    let mut min_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MinCapacity" => {
                                min_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingConstraints {
                        max_capacity: max_capacity.ok_or(::serde::de::Error::missing_field("MaxCapacity"))?,
                        min_capacity: min_capacity.ok_or(::serde::de::Error::missing_field("MinCapacity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.ScalingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html) property type.
    #[derive(Debug)]
    pub struct ScalingRule {
        /// Property `Action`.
        pub action: ::Value<ScalingAction>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Trigger`.
        pub trigger: ::Value<ScalingTrigger>,
    }

    impl ::codec::SerializeValue for ScalingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trigger", &self.trigger)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action = None;
                    let mut description = None;
                    let mut name = None;
                    let mut trigger = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Trigger" => {
                                trigger = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingRule {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        description: description,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        trigger: trigger.ok_or(::serde::de::Error::missing_field("Trigger"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.ScalingTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingtrigger.html) property type.
    #[derive(Debug)]
    pub struct ScalingTrigger {
        /// Property `CloudWatchAlarmDefinition`.
        pub cloud_watch_alarm_definition: ::Value<CloudWatchAlarmDefinition>,
    }

    impl ::codec::SerializeValue for ScalingTrigger {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchAlarmDefinition", &self.cloud_watch_alarm_definition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingTrigger {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingTrigger, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingTrigger;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingTrigger")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_alarm_definition = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchAlarmDefinition" => {
                                cloud_watch_alarm_definition = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingTrigger {
                        cloud_watch_alarm_definition: cloud_watch_alarm_definition.ok_or(::serde::de::Error::missing_field("CloudWatchAlarmDefinition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.ScriptBootstrapActionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scriptbootstrapactionconfig.html) property type.
    #[derive(Debug)]
    pub struct ScriptBootstrapActionConfig {
        /// Property `Args`.
        pub args: Option<::ValueList<String>>,
        /// Property `Path`.
        pub path: ::Value<String>,
    }

    impl ::codec::SerializeValue for ScriptBootstrapActionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Args", &self.args)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScriptBootstrapActionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScriptBootstrapActionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScriptBootstrapActionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScriptBootstrapActionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut args = None;
                    let mut path = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Args" => {
                                args = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Path" => {
                                path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScriptBootstrapActionConfig {
                        args: args,
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html) property type.
    #[derive(Debug)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        pub adjustment_type: Option<::Value<String>>,
        /// Property `CoolDown`.
        pub cool_down: Option<::Value<u32>>,
        /// Property `ScalingAdjustment`.
        pub scaling_adjustment: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SimpleScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", &self.adjustment_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoolDown", &self.cool_down)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", &self.scaling_adjustment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SimpleScalingPolicyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SimpleScalingPolicyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SimpleScalingPolicyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SimpleScalingPolicyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adjustment_type = None;
                    let mut cool_down = None;
                    let mut scaling_adjustment = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdjustmentType" => {
                                adjustment_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CoolDown" => {
                                cool_down = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SimpleScalingPolicyConfiguration {
                        adjustment_type: adjustment_type,
                        cool_down: cool_down,
                        scaling_adjustment: scaling_adjustment.ok_or(::serde::de::Error::missing_field("ScalingAdjustment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.SpotProvisioningSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html) property type.
    #[derive(Debug)]
    pub struct SpotProvisioningSpecification {
        /// Property `BlockDurationMinutes`.
        pub block_duration_minutes: Option<::Value<u32>>,
        /// Property `TimeoutAction`.
        pub timeout_action: ::Value<String>,
        /// Property `TimeoutDurationMinutes`.
        pub timeout_duration_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SpotProvisioningSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDurationMinutes", &self.block_duration_minutes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutAction", &self.timeout_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutDurationMinutes", &self.timeout_duration_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotProvisioningSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotProvisioningSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotProvisioningSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotProvisioningSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_duration_minutes = None;
                    let mut timeout_action = None;
                    let mut timeout_duration_minutes = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockDurationMinutes" => {
                                block_duration_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TimeoutAction" => {
                                timeout_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TimeoutDurationMinutes" => {
                                timeout_duration_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotProvisioningSpecification {
                        block_duration_minutes: block_duration_minutes,
                        timeout_action: timeout_action.ok_or(::serde::de::Error::missing_field("TimeoutAction"))?,
                        timeout_duration_minutes: timeout_duration_minutes.ok_or(::serde::de::Error::missing_field("TimeoutDurationMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Cluster.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html) property type.
    #[derive(Debug)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SizeInGB`.
        pub size_in_gb: ::Value<u32>,
        /// Property `VolumeType`.
        pub volume_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for VolumeSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGB", &self.size_in_gb)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VolumeSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumeSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumeSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iops = None;
                    let mut size_in_gb = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SizeInGB" => {
                                size_in_gb = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumeSpecification {
                        iops: iops,
                        size_in_gb: size_in_gb.ok_or(::serde::de::Error::missing_field("SizeInGB"))?,
                        volume_type: volume_type.ok_or(::serde::de::Error::missing_field("VolumeType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance_fleet_config {
    //! Property types for the `InstanceFleetConfig` resource.

    /// The [`AWS::EMR::InstanceFleetConfig.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html) property type.
    #[derive(Debug)]
    pub struct Configuration {
        /// Property `Classification`.
        pub classification: Option<::Value<String>>,
        /// Property `ConfigurationProperties`.
        pub configuration_properties: Option<::ValueMap<String>>,
        /// Property `Configurations`.
        pub configurations: Option<::ValueList<Configuration>>,
    }

    impl ::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", &self.classification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", &self.configuration_properties)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Configuration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Configuration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Configuration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Configuration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classification = None;
                    let mut configuration_properties = None;
                    let mut configurations = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ConfigurationProperties" => {
                                configuration_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configurations" => {
                                configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Configuration {
                        classification: classification,
                        configuration_properties: configuration_properties,
                        configurations: configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceFleetConfig.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig.html) property type.
    #[derive(Debug)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property `VolumesPerInstance`.
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EbsBlockDeviceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSpecification", &self.volume_specification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesPerInstance", &self.volumes_per_instance)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsBlockDeviceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsBlockDeviceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsBlockDeviceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsBlockDeviceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut volume_specification = None;
                    let mut volumes_per_instance = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VolumeSpecification" => {
                                volume_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumesPerInstance" => {
                                volumes_per_instance = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsBlockDeviceConfig {
                        volume_specification: volume_specification.ok_or(::serde::de::Error::missing_field("VolumeSpecification"))?,
                        volumes_per_instance: volumes_per_instance,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceFleetConfig.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property `EbsOptimized`.
        pub ebs_optimized: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EbsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsBlockDeviceConfigs", &self.ebs_block_device_configs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ebs_block_device_configs = None;
                    let mut ebs_optimized = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EbsBlockDeviceConfigs" => {
                                ebs_block_device_configs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsOptimized" => {
                                ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsConfiguration {
                        ebs_block_device_configs: ebs_block_device_configs,
                        ebs_optimized: ebs_optimized,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceFleetConfig.InstanceFleetProvisioningSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancefleetprovisioningspecifications.html) property type.
    #[derive(Debug)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property `SpotSpecification`.
        pub spot_specification: ::Value<SpotProvisioningSpecification>,
    }

    impl ::codec::SerializeValue for InstanceFleetProvisioningSpecifications {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotSpecification", &self.spot_specification)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceFleetProvisioningSpecifications {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceFleetProvisioningSpecifications, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceFleetProvisioningSpecifications;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceFleetProvisioningSpecifications")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut spot_specification = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpotSpecification" => {
                                spot_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceFleetProvisioningSpecifications {
                        spot_specification: spot_specification.ok_or(::serde::de::Error::missing_field("SpotSpecification"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceFleetConfig.InstanceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html) property type.
    #[derive(Debug)]
    pub struct InstanceTypeConfig {
        /// Property `BidPrice`.
        pub bid_price: Option<::Value<String>>,
        /// Property `BidPriceAsPercentageOfOnDemandPrice`.
        pub bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>>,
        /// Property `Configurations`.
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property `EbsConfiguration`.
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property `InstanceType`.
        pub instance_type: ::Value<String>,
        /// Property `WeightedCapacity`.
        pub weighted_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InstanceTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", &self.bid_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPriceAsPercentageOfOnDemandPrice", &self.bid_price_as_percentage_of_on_demand_price)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", &self.ebs_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", &self.weighted_capacity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceTypeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceTypeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceTypeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceTypeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bid_price = None;
                    let mut bid_price_as_percentage_of_on_demand_price = None;
                    let mut configurations = None;
                    let mut ebs_configuration = None;
                    let mut instance_type = None;
                    let mut weighted_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BidPrice" => {
                                bid_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BidPriceAsPercentageOfOnDemandPrice" => {
                                bid_price_as_percentage_of_on_demand_price = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configurations" => {
                                configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsConfiguration" => {
                                ebs_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceType" => {
                                instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceTypeConfig {
                        bid_price: bid_price,
                        bid_price_as_percentage_of_on_demand_price: bid_price_as_percentage_of_on_demand_price,
                        configurations: configurations,
                        ebs_configuration: ebs_configuration,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        weighted_capacity: weighted_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceFleetConfig.SpotProvisioningSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html) property type.
    #[derive(Debug)]
    pub struct SpotProvisioningSpecification {
        /// Property `BlockDurationMinutes`.
        pub block_duration_minutes: Option<::Value<u32>>,
        /// Property `TimeoutAction`.
        pub timeout_action: ::Value<String>,
        /// Property `TimeoutDurationMinutes`.
        pub timeout_duration_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SpotProvisioningSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDurationMinutes", &self.block_duration_minutes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutAction", &self.timeout_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutDurationMinutes", &self.timeout_duration_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpotProvisioningSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpotProvisioningSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpotProvisioningSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpotProvisioningSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_duration_minutes = None;
                    let mut timeout_action = None;
                    let mut timeout_duration_minutes = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockDurationMinutes" => {
                                block_duration_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TimeoutAction" => {
                                timeout_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TimeoutDurationMinutes" => {
                                timeout_duration_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SpotProvisioningSpecification {
                        block_duration_minutes: block_duration_minutes,
                        timeout_action: timeout_action.ok_or(::serde::de::Error::missing_field("TimeoutAction"))?,
                        timeout_duration_minutes: timeout_duration_minutes.ok_or(::serde::de::Error::missing_field("TimeoutDurationMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceFleetConfig.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html) property type.
    #[derive(Debug)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SizeInGB`.
        pub size_in_gb: ::Value<u32>,
        /// Property `VolumeType`.
        pub volume_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for VolumeSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGB", &self.size_in_gb)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VolumeSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumeSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumeSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iops = None;
                    let mut size_in_gb = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SizeInGB" => {
                                size_in_gb = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumeSpecification {
                        iops: iops,
                        size_in_gb: size_in_gb.ok_or(::serde::de::Error::missing_field("SizeInGB"))?,
                        volume_type: volume_type.ok_or(::serde::de::Error::missing_field("VolumeType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance_group_config {
    //! Property types for the `InstanceGroupConfig` resource.

    /// The [`AWS::EMR::InstanceGroupConfig.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-autoscalingpolicy.html) property type.
    #[derive(Debug)]
    pub struct AutoScalingPolicy {
        /// Property `Constraints`.
        pub constraints: ::Value<ScalingConstraints>,
        /// Property `Rules`.
        pub rules: ::ValueList<ScalingRule>,
    }

    impl ::codec::SerializeValue for AutoScalingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", &self.constraints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoScalingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoScalingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoScalingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints = None;
                    let mut rules = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Rules" => {
                                rules = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoScalingPolicy {
                        constraints: constraints.ok_or(::serde::de::Error::missing_field("Constraints"))?,
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html) property type.
    #[derive(Debug)]
    pub struct CloudWatchAlarmDefinition {
        /// Property `ComparisonOperator`.
        pub comparison_operator: ::Value<String>,
        /// Property `Dimensions`.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `EvaluationPeriods`.
        pub evaluation_periods: Option<::Value<u32>>,
        /// Property `MetricName`.
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        pub namespace: Option<::Value<String>>,
        /// Property `Period`.
        pub period: ::Value<u32>,
        /// Property `Statistic`.
        pub statistic: Option<::Value<String>>,
        /// Property `Threshold`.
        pub threshold: ::Value<f64>,
        /// Property `Unit`.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchAlarmDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", &self.evaluation_periods)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchAlarmDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchAlarmDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchAlarmDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchAlarmDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator = None;
                    let mut dimensions = None;
                    let mut evaluation_periods = None;
                    let mut metric_name = None;
                    let mut namespace = None;
                    let mut period = None;
                    let mut statistic = None;
                    let mut threshold = None;
                    let mut unit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Dimensions" => {
                                dimensions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EvaluationPeriods" => {
                                evaluation_periods = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricName" => {
                                metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Namespace" => {
                                namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Period" => {
                                period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Statistic" => {
                                statistic = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Threshold" => {
                                threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Unit" => {
                                unit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchAlarmDefinition {
                        comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                        dimensions: dimensions,
                        evaluation_periods: evaluation_periods,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        namespace: namespace,
                        period: period.ok_or(::serde::de::Error::missing_field("Period"))?,
                        statistic: statistic,
                        threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html) property type.
    #[derive(Debug)]
    pub struct Configuration {
        /// Property `Classification`.
        pub classification: Option<::Value<String>>,
        /// Property `ConfigurationProperties`.
        pub configuration_properties: Option<::ValueMap<String>>,
        /// Property `Configurations`.
        pub configurations: Option<::ValueList<Configuration>>,
    }

    impl ::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", &self.classification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", &self.configuration_properties)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", &self.configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Configuration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Configuration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Configuration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Configuration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classification = None;
                    let mut configuration_properties = None;
                    let mut configurations = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ConfigurationProperties" => {
                                configuration_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Configurations" => {
                                configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Configuration {
                        classification: classification,
                        configuration_properties: configuration_properties,
                        configurations: configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig.html) property type.
    #[derive(Debug)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property `VolumesPerInstance`.
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EbsBlockDeviceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSpecification", &self.volume_specification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesPerInstance", &self.volumes_per_instance)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsBlockDeviceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsBlockDeviceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsBlockDeviceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsBlockDeviceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut volume_specification = None;
                    let mut volumes_per_instance = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VolumeSpecification" => {
                                volume_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumesPerInstance" => {
                                volumes_per_instance = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsBlockDeviceConfig {
                        volume_specification: volume_specification.ok_or(::serde::de::Error::missing_field("VolumeSpecification"))?,
                        volumes_per_instance: volumes_per_instance,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property `EbsOptimized`.
        pub ebs_optimized: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EbsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsBlockDeviceConfigs", &self.ebs_block_device_configs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ebs_block_device_configs = None;
                    let mut ebs_optimized = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EbsBlockDeviceConfigs" => {
                                ebs_block_device_configs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EbsOptimized" => {
                                ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsConfiguration {
                        ebs_block_device_configs: ebs_block_device_configs,
                        ebs_optimized: ebs_optimized,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-metricdimension.html) property type.
    #[derive(Debug)]
    pub struct MetricDimension {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
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
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDimension {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingaction.html) property type.
    #[derive(Debug)]
    pub struct ScalingAction {
        /// Property `Market`.
        pub market: Option<::Value<String>>,
        /// Property `SimpleScalingPolicyConfiguration`.
        pub simple_scaling_policy_configuration: ::Value<SimpleScalingPolicyConfiguration>,
    }

    impl ::codec::SerializeValue for ScalingAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", &self.market)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SimpleScalingPolicyConfiguration", &self.simple_scaling_policy_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut market = None;
                    let mut simple_scaling_policy_configuration = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Market" => {
                                market = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SimpleScalingPolicyConfiguration" => {
                                simple_scaling_policy_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingAction {
                        market: market,
                        simple_scaling_policy_configuration: simple_scaling_policy_configuration.ok_or(::serde::de::Error::missing_field("SimpleScalingPolicyConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingconstraints.html) property type.
    #[derive(Debug)]
    pub struct ScalingConstraints {
        /// Property `MaxCapacity`.
        pub max_capacity: ::Value<u32>,
        /// Property `MinCapacity`.
        pub min_capacity: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ScalingConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", &self.max_capacity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", &self.min_capacity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingConstraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingConstraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingConstraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingConstraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_capacity = None;
                    let mut min_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MinCapacity" => {
                                min_capacity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingConstraints {
                        max_capacity: max_capacity.ok_or(::serde::de::Error::missing_field("MaxCapacity"))?,
                        min_capacity: min_capacity.ok_or(::serde::de::Error::missing_field("MinCapacity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html) property type.
    #[derive(Debug)]
    pub struct ScalingRule {
        /// Property `Action`.
        pub action: ::Value<ScalingAction>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Trigger`.
        pub trigger: ::Value<ScalingTrigger>,
    }

    impl ::codec::SerializeValue for ScalingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trigger", &self.trigger)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action = None;
                    let mut description = None;
                    let mut name = None;
                    let mut trigger = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Trigger" => {
                                trigger = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingRule {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        description: description,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        trigger: trigger.ok_or(::serde::de::Error::missing_field("Trigger"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingtrigger.html) property type.
    #[derive(Debug)]
    pub struct ScalingTrigger {
        /// Property `CloudWatchAlarmDefinition`.
        pub cloud_watch_alarm_definition: ::Value<CloudWatchAlarmDefinition>,
    }

    impl ::codec::SerializeValue for ScalingTrigger {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchAlarmDefinition", &self.cloud_watch_alarm_definition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingTrigger {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingTrigger, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingTrigger;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingTrigger")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_alarm_definition = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchAlarmDefinition" => {
                                cloud_watch_alarm_definition = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingTrigger {
                        cloud_watch_alarm_definition: cloud_watch_alarm_definition.ok_or(::serde::de::Error::missing_field("CloudWatchAlarmDefinition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html) property type.
    #[derive(Debug)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        pub adjustment_type: Option<::Value<String>>,
        /// Property `CoolDown`.
        pub cool_down: Option<::Value<u32>>,
        /// Property `ScalingAdjustment`.
        pub scaling_adjustment: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SimpleScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", &self.adjustment_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoolDown", &self.cool_down)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", &self.scaling_adjustment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SimpleScalingPolicyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SimpleScalingPolicyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SimpleScalingPolicyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SimpleScalingPolicyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adjustment_type = None;
                    let mut cool_down = None;
                    let mut scaling_adjustment = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdjustmentType" => {
                                adjustment_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CoolDown" => {
                                cool_down = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SimpleScalingPolicyConfiguration {
                        adjustment_type: adjustment_type,
                        cool_down: cool_down,
                        scaling_adjustment: scaling_adjustment.ok_or(::serde::de::Error::missing_field("ScalingAdjustment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::InstanceGroupConfig.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html) property type.
    #[derive(Debug)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SizeInGB`.
        pub size_in_gb: ::Value<u32>,
        /// Property `VolumeType`.
        pub volume_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for VolumeSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGB", &self.size_in_gb)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VolumeSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumeSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumeSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iops = None;
                    let mut size_in_gb = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SizeInGB" => {
                                size_in_gb = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumeSpecification {
                        iops: iops,
                        size_in_gb: size_in_gb.ok_or(::serde::de::Error::missing_field("SizeInGB"))?,
                        volume_type: volume_type.ok_or(::serde::de::Error::missing_field("VolumeType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod step {
    //! Property types for the `Step` resource.

    /// The [`AWS::EMR::Step.HadoopJarStepConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html) property type.
    #[derive(Debug)]
    pub struct HadoopJarStepConfig {
        /// Property `Args`.
        pub args: Option<::ValueList<String>>,
        /// Property `Jar`.
        pub jar: ::Value<String>,
        /// Property `MainClass`.
        pub main_class: Option<::Value<String>>,
        /// Property `StepProperties`.
        pub step_properties: Option<::ValueList<KeyValue>>,
    }

    impl ::codec::SerializeValue for HadoopJarStepConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Args", &self.args)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Jar", &self.jar)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MainClass", &self.main_class)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepProperties", &self.step_properties)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HadoopJarStepConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HadoopJarStepConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HadoopJarStepConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HadoopJarStepConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut args = None;
                    let mut jar = None;
                    let mut main_class = None;
                    let mut step_properties = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Args" => {
                                args = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Jar" => {
                                jar = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MainClass" => {
                                main_class = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StepProperties" => {
                                step_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HadoopJarStepConfig {
                        args: args,
                        jar: jar.ok_or(::serde::de::Error::missing_field("Jar"))?,
                        main_class: main_class,
                        step_properties: step_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMR::Step.KeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-keyvalue.html) property type.
    #[derive(Debug)]
    pub struct KeyValue {
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KeyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyValue {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
