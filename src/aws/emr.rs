//! Types for the `EMR` service.

/// The [`AWS::EMR::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`AdditionalInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-additionalinfo).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_info: Option<::Value<::json::Value>>,
    /// Property [`Applications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-applications).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub applications: Option<::ValueList<self::cluster::Application>>,
    /// Property [`AutoScalingRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-autoscalingrole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auto_scaling_role: Option<::Value<String>>,
    /// Property [`BootstrapActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-bootstrapactions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bootstrap_actions: Option<::ValueList<self::cluster::BootstrapActionConfig>>,
    /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-configurations).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configurations: Option<::ValueList<self::cluster::Configuration>>,
    /// Property [`CustomAmiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-customamiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub custom_ami_id: Option<::Value<String>>,
    /// Property [`EbsRootVolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-ebsrootvolumesize).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ebs_root_volume_size: Option<::Value<u32>>,
    /// Property [`Instances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-instances).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub instances: ::Value<self::cluster::JobFlowInstancesConfig>,
    /// Property [`JobFlowRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-jobflowrole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_flow_role: ::Value<String>,
    /// Property [`LogUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-loguri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_uri: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ReleaseLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-releaselabel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub release_label: Option<::Value<String>>,
    /// Property [`ScaleDownBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-scaledownbehavior).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scale_down_behavior: Option<::Value<String>>,
    /// Property [`SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-securityconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_configuration: Option<::Value<String>>,
    /// Property [`ServiceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-servicerole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_role: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VisibleToAllUsers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html#cfn-elasticmapreduce-cluster-visibletoallusers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub visible_to_all_users: Option<::Value<bool>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_info) = self.additional_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInfo", additional_info)?;
        }
        if let Some(ref applications) = self.applications {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Applications", applications)?;
        }
        if let Some(ref auto_scaling_role) = self.auto_scaling_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingRole", auto_scaling_role)?;
        }
        if let Some(ref bootstrap_actions) = self.bootstrap_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BootstrapActions", bootstrap_actions)?;
        }
        if let Some(ref configurations) = self.configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
        }
        if let Some(ref custom_ami_id) = self.custom_ami_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomAmiId", custom_ami_id)?;
        }
        if let Some(ref ebs_root_volume_size) = self.ebs_root_volume_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsRootVolumeSize", ebs_root_volume_size)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instances", &self.instances)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobFlowRole", &self.job_flow_role)?;
        if let Some(ref log_uri) = self.log_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogUri", log_uri)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref release_label) = self.release_label {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReleaseLabel", release_label)?;
        }
        if let Some(ref scale_down_behavior) = self.scale_down_behavior {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleDownBehavior", scale_down_behavior)?;
        }
        if let Some(ref security_configuration) = self.security_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityConfiguration", security_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref visible_to_all_users) = self.visible_to_all_users {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibleToAllUsers", visible_to_all_users)?;
        }
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
                let mut additional_info: Option<::Value<::json::Value>> = None;
                let mut applications: Option<::ValueList<self::cluster::Application>> = None;
                let mut auto_scaling_role: Option<::Value<String>> = None;
                let mut bootstrap_actions: Option<::ValueList<self::cluster::BootstrapActionConfig>> = None;
                let mut configurations: Option<::ValueList<self::cluster::Configuration>> = None;
                let mut custom_ami_id: Option<::Value<String>> = None;
                let mut ebs_root_volume_size: Option<::Value<u32>> = None;
                let mut instances: Option<::Value<self::cluster::JobFlowInstancesConfig>> = None;
                let mut job_flow_role: Option<::Value<String>> = None;
                let mut log_uri: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut release_label: Option<::Value<String>> = None;
                let mut scale_down_behavior: Option<::Value<String>> = None;
                let mut security_configuration: Option<::Value<String>> = None;
                let mut service_role: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut visible_to_all_users: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalInfo" => {
                            additional_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Applications" => {
                            applications = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoScalingRole" => {
                            auto_scaling_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BootstrapActions" => {
                            bootstrap_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configurations" => {
                            configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomAmiId" => {
                            custom_ami_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EbsRootVolumeSize" => {
                            ebs_root_volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Instances" => {
                            instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobFlowRole" => {
                            job_flow_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogUri" => {
                            log_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReleaseLabel" => {
                            release_label = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScaleDownBehavior" => {
                            scale_down_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityConfiguration" => {
                            security_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRole" => {
                            service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VisibleToAllUsers" => {
                            visible_to_all_users = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct InstanceFleetConfig {
    properties: InstanceFleetConfigProperties
}

/// Properties for the `InstanceFleetConfig` resource.
#[derive(Debug, Default)]
pub struct InstanceFleetConfigProperties {
    /// Property [`ClusterId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-clusterid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_id: ::Value<String>,
    /// Property [`InstanceFleetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancefleettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_fleet_type: ::Value<String>,
    /// Property [`InstanceTypeConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfigs).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_type_configs: Option<::ValueList<self::instance_fleet_config::InstanceTypeConfig>>,
    /// Property [`LaunchSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-launchspecifications).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub launch_specifications: Option<::Value<self::instance_fleet_config::InstanceFleetProvisioningSpecifications>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`TargetOnDemandCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-targetondemandcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_on_demand_capacity: Option<::Value<u32>>,
    /// Property [`TargetSpotCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html#cfn-elasticmapreduce-instancefleetconfig-targetspotcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_spot_capacity: Option<::Value<u32>>,
}

impl ::serde::Serialize for InstanceFleetConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterId", &self.cluster_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceFleetType", &self.instance_fleet_type)?;
        if let Some(ref instance_type_configs) = self.instance_type_configs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypeConfigs", instance_type_configs)?;
        }
        if let Some(ref launch_specifications) = self.launch_specifications {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchSpecifications", launch_specifications)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref target_on_demand_capacity) = self.target_on_demand_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOnDemandCapacity", target_on_demand_capacity)?;
        }
        if let Some(ref target_spot_capacity) = self.target_spot_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetSpotCapacity", target_spot_capacity)?;
        }
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
                let mut cluster_id: Option<::Value<String>> = None;
                let mut instance_fleet_type: Option<::Value<String>> = None;
                let mut instance_type_configs: Option<::ValueList<self::instance_fleet_config::InstanceTypeConfig>> = None;
                let mut launch_specifications: Option<::Value<self::instance_fleet_config::InstanceFleetProvisioningSpecifications>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut target_on_demand_capacity: Option<::Value<u32>> = None;
                let mut target_spot_capacity: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterId" => {
                            cluster_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceFleetType" => {
                            instance_fleet_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceTypeConfigs" => {
                            instance_type_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchSpecifications" => {
                            launch_specifications = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetOnDemandCapacity" => {
                            target_on_demand_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetSpotCapacity" => {
                            target_spot_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct InstanceGroupConfig {
    properties: InstanceGroupConfigProperties
}

/// Properties for the `InstanceGroupConfig` resource.
#[derive(Debug, Default)]
pub struct InstanceGroupConfigProperties {
    /// Property [`AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-elasticmapreduce-instancegroupconfig-autoscalingpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_scaling_policy: Option<::Value<self::instance_group_config::AutoScalingPolicy>>,
    /// Property [`BidPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-bidprice).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bid_price: Option<::Value<String>>,
    /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-configurations).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configurations: Option<::ValueList<self::instance_group_config::Configuration>>,
    /// Property [`EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-ebsconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ebs_configuration: Option<::Value<self::instance_group_config::EbsConfiguration>>,
    /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfiginstancecount-).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_count: ::Value<u32>,
    /// Property [`InstanceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-instancerole).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_role: ::Value<String>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-instancetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`JobFlowId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-jobflowid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_flow_id: ::Value<String>,
    /// Property [`Market`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-market).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub market: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html#cfn-emr-instancegroupconfig-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for InstanceGroupConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_scaling_policy) = self.auto_scaling_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingPolicy", auto_scaling_policy)?;
        }
        if let Some(ref bid_price) = self.bid_price {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", bid_price)?;
        }
        if let Some(ref configurations) = self.configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
        }
        if let Some(ref ebs_configuration) = self.ebs_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", ebs_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceRole", &self.instance_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobFlowId", &self.job_flow_id)?;
        if let Some(ref market) = self.market {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", market)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
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
                let mut auto_scaling_policy: Option<::Value<self::instance_group_config::AutoScalingPolicy>> = None;
                let mut bid_price: Option<::Value<String>> = None;
                let mut configurations: Option<::ValueList<self::instance_group_config::Configuration>> = None;
                let mut ebs_configuration: Option<::Value<self::instance_group_config::EbsConfiguration>> = None;
                let mut instance_count: Option<::Value<u32>> = None;
                let mut instance_role: Option<::Value<String>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut job_flow_id: Option<::Value<String>> = None;
                let mut market: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingPolicy" => {
                            auto_scaling_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BidPrice" => {
                            bid_price = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configurations" => {
                            configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EbsConfiguration" => {
                            ebs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceCount" => {
                            instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceRole" => {
                            instance_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobFlowId" => {
                            job_flow_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Market" => {
                            market = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct SecurityConfiguration {
    properties: SecurityConfigurationProperties
}

/// Properties for the `SecurityConfiguration` resource.
#[derive(Debug, Default)]
pub struct SecurityConfigurationProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-securityconfiguration.html#cfn-emr-securityconfiguration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-securityconfiguration.html#cfn-emr-securityconfiguration-securityconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_configuration: ::Value<::json::Value>,
}

impl ::serde::Serialize for SecurityConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
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
                let mut name: Option<::Value<String>> = None;
                let mut security_configuration: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityConfiguration" => {
                            security_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct Step {
    properties: StepProperties
}

/// Properties for the `Step` resource.
#[derive(Debug, Default)]
pub struct StepProperties {
    /// Property [`ActionOnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html#cfn-elasticmapreduce-step-actiononfailure).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action_on_failure: ::Value<String>,
    /// Property [`HadoopJarStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html#cfn-elasticmapreduce-step-hadoopjarstep).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub hadoop_jar_step: ::Value<self::step::HadoopJarStepConfig>,
    /// Property [`JobFlowId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html#cfn-elasticmapreduce-step-jobflowid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_flow_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html#cfn-elasticmapreduce-step-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
                let mut action_on_failure: Option<::Value<String>> = None;
                let mut hadoop_jar_step: Option<::Value<self::step::HadoopJarStepConfig>> = None;
                let mut job_flow_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionOnFailure" => {
                            action_on_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HadoopJarStep" => {
                            hadoop_jar_step = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobFlowId" => {
                            job_flow_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Application {
        /// Property [`AdditionalInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-application.html#cfn-elasticmapreduce-cluster-application-additionalinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub additional_info: Option<::ValueMap<String>>,
        /// Property [`Args`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-application.html#cfn-elasticmapreduce-cluster-application-args).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub args: Option<::ValueList<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-application.html#cfn-elasticmapreduce-cluster-application-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-application.html#cfn-elasticmapreduce-cluster-application-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Application {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref additional_info) = self.additional_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInfo", additional_info)?;
            }
            if let Some(ref args) = self.args {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Args", args)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
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
                    let mut additional_info: Option<::ValueMap<String>> = None;
                    let mut args: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalInfo" => {
                                additional_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Args" => {
                                args = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct AutoScalingPolicy {
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoscalingpolicy.html#cfn-elasticmapreduce-cluster-autoscalingpolicy-constraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constraints: ::Value<ScalingConstraints>,
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoscalingpolicy.html#cfn-elasticmapreduce-cluster-autoscalingpolicy-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut constraints: Option<::Value<ScalingConstraints>> = None;
                    let mut rules: Option<::ValueList<ScalingRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct BootstrapActionConfig {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-bootstrapactionconfig.html#cfn-elasticmapreduce-cluster-bootstrapactionconfig-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ScriptBootstrapAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-bootstrapactionconfig.html#cfn-elasticmapreduce-cluster-bootstrapactionconfig-scriptbootstrapaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut name: Option<::Value<String>> = None;
                    let mut script_bootstrap_action: Option<::Value<ScriptBootstrapActionConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScriptBootstrapAction" => {
                                script_bootstrap_action = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct CloudWatchAlarmDefinition {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: ::Value<String>,
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property [`EvaluationPeriods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-evaluationperiods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub evaluation_periods: Option<::Value<u32>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<::Value<String>>,
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: ::Value<u32>,
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: Option<::Value<String>>,
        /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-threshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold: ::Value<f64>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-cluster-cloudwatchalarmdefinition-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchAlarmDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            if let Some(ref evaluation_periods) = self.evaluation_periods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", evaluation_periods)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            if let Some(ref statistic) = self.statistic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", statistic)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
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
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut dimensions: Option<::ValueList<MetricDimension>> = None;
                    let mut evaluation_periods: Option<::Value<u32>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut namespace: Option<::Value<String>> = None;
                    let mut period: Option<::Value<u32>> = None;
                    let mut statistic: Option<::Value<String>> = None;
                    let mut threshold: Option<::Value<f64>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EvaluationPeriods" => {
                                evaluation_periods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistic" => {
                                statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Threshold" => {
                                threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Configuration {
        /// Property [`Classification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html#cfn-elasticmapreduce-cluster-configuration-classification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub classification: Option<::Value<String>>,
        /// Property [`ConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html#cfn-elasticmapreduce-cluster-configuration-configurationproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_properties: Option<::ValueMap<String>>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html#cfn-elasticmapreduce-cluster-configuration-configurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configurations: Option<::ValueList<Configuration>>,
    }

    impl ::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref classification) = self.classification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", classification)?;
            }
            if let Some(ref configuration_properties) = self.configuration_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", configuration_properties)?;
            }
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
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
                    let mut classification: Option<::Value<String>> = None;
                    let mut configuration_properties: Option<::ValueMap<String>> = None;
                    let mut configurations: Option<::ValueList<Configuration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigurationProperties" => {
                                configuration_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsBlockDeviceConfig {
        /// Property [`VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsblockdeviceconfig.html#cfn-elasticmapreduce-cluster-ebsblockdeviceconfig-volumespecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property [`VolumesPerInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsblockdeviceconfig.html#cfn-elasticmapreduce-cluster-ebsblockdeviceconfig-volumesperinstance).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EbsBlockDeviceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSpecification", &self.volume_specification)?;
            if let Some(ref volumes_per_instance) = self.volumes_per_instance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesPerInstance", volumes_per_instance)?;
            }
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
                    let mut volume_specification: Option<::Value<VolumeSpecification>> = None;
                    let mut volumes_per_instance: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VolumeSpecification" => {
                                volume_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumesPerInstance" => {
                                volumes_per_instance = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsConfiguration {
        /// Property [`EbsBlockDeviceConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsconfiguration.html#cfn-elasticmapreduce-cluster-ebsconfiguration-ebsblockdeviceconfigs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property [`EbsOptimized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsconfiguration.html#cfn-elasticmapreduce-cluster-ebsconfiguration-ebsoptimized).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_optimized: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EbsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ebs_block_device_configs) = self.ebs_block_device_configs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsBlockDeviceConfigs", ebs_block_device_configs)?;
            }
            if let Some(ref ebs_optimized) = self.ebs_optimized {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", ebs_optimized)?;
            }
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
                    let mut ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>> = None;
                    let mut ebs_optimized: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EbsBlockDeviceConfigs" => {
                                ebs_block_device_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbsOptimized" => {
                                ebs_optimized = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct InstanceFleetConfig {
        /// Property [`InstanceTypeConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html#cfn-elasticmapreduce-cluster-instancefleetconfig-instancetypeconfigs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type_configs: Option<::ValueList<InstanceTypeConfig>>,
        /// Property [`LaunchSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html#cfn-elasticmapreduce-cluster-instancefleetconfig-launchspecifications).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub launch_specifications: Option<::Value<InstanceFleetProvisioningSpecifications>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html#cfn-elasticmapreduce-cluster-instancefleetconfig-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`TargetOnDemandCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html#cfn-elasticmapreduce-cluster-instancefleetconfig-targetondemandcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_on_demand_capacity: Option<::Value<u32>>,
        /// Property [`TargetSpotCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html#cfn-elasticmapreduce-cluster-instancefleetconfig-targetspotcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_spot_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InstanceFleetConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_type_configs) = self.instance_type_configs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypeConfigs", instance_type_configs)?;
            }
            if let Some(ref launch_specifications) = self.launch_specifications {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchSpecifications", launch_specifications)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref target_on_demand_capacity) = self.target_on_demand_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOnDemandCapacity", target_on_demand_capacity)?;
            }
            if let Some(ref target_spot_capacity) = self.target_spot_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetSpotCapacity", target_spot_capacity)?;
            }
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
                    let mut instance_type_configs: Option<::ValueList<InstanceTypeConfig>> = None;
                    let mut launch_specifications: Option<::Value<InstanceFleetProvisioningSpecifications>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut target_on_demand_capacity: Option<::Value<u32>> = None;
                    let mut target_spot_capacity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceTypeConfigs" => {
                                instance_type_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchSpecifications" => {
                                launch_specifications = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOnDemandCapacity" => {
                                target_on_demand_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetSpotCapacity" => {
                                target_spot_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property [`SpotSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetprovisioningspecifications.html#cfn-elasticmapreduce-cluster-instancefleetprovisioningspecifications-spotspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut spot_specification: Option<::Value<SpotProvisioningSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpotSpecification" => {
                                spot_specification = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct InstanceGroupConfig {
        /// Property [`AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-autoscalingpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_scaling_policy: Option<::Value<AutoScalingPolicy>>,
        /// Property [`BidPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-bidprice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bid_price: Option<::Value<String>>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-configurations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property [`EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-ebsconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-instancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`Market`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-market).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub market: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html#cfn-elasticmapreduce-cluster-instancegroupconfig-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceGroupConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_scaling_policy) = self.auto_scaling_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingPolicy", auto_scaling_policy)?;
            }
            if let Some(ref bid_price) = self.bid_price {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", bid_price)?;
            }
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
            if let Some(ref ebs_configuration) = self.ebs_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", ebs_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref market) = self.market {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", market)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
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
                    let mut auto_scaling_policy: Option<::Value<AutoScalingPolicy>> = None;
                    let mut bid_price: Option<::Value<String>> = None;
                    let mut configurations: Option<::ValueList<Configuration>> = None;
                    let mut ebs_configuration: Option<::Value<EbsConfiguration>> = None;
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut market: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoScalingPolicy" => {
                                auto_scaling_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BidPrice" => {
                                bid_price = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbsConfiguration" => {
                                ebs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Market" => {
                                market = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct InstanceTypeConfig {
        /// Property [`BidPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html#cfn-elasticmapreduce-cluster-instancetypeconfig-bidprice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bid_price: Option<::Value<String>>,
        /// Property [`BidPriceAsPercentageOfOnDemandPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html#cfn-elasticmapreduce-cluster-instancetypeconfig-bidpriceaspercentageofondemandprice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html#cfn-elasticmapreduce-cluster-instancetypeconfig-configurations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property [`EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html#cfn-elasticmapreduce-cluster-instancetypeconfig-ebsconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html#cfn-elasticmapreduce-cluster-instancetypeconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`WeightedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html#cfn-elasticmapreduce-cluster-instancetypeconfig-weightedcapacity).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub weighted_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InstanceTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bid_price) = self.bid_price {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", bid_price)?;
            }
            if let Some(ref bid_price_as_percentage_of_on_demand_price) = self.bid_price_as_percentage_of_on_demand_price {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPriceAsPercentageOfOnDemandPrice", bid_price_as_percentage_of_on_demand_price)?;
            }
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
            if let Some(ref ebs_configuration) = self.ebs_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", ebs_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref weighted_capacity) = self.weighted_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", weighted_capacity)?;
            }
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
                    let mut bid_price: Option<::Value<String>> = None;
                    let mut bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>> = None;
                    let mut configurations: Option<::ValueList<Configuration>> = None;
                    let mut ebs_configuration: Option<::Value<EbsConfiguration>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut weighted_capacity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BidPrice" => {
                                bid_price = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BidPriceAsPercentageOfOnDemandPrice" => {
                                bid_price_as_percentage_of_on_demand_price = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbsConfiguration" => {
                                ebs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct JobFlowInstancesConfig {
        /// Property [`AdditionalMasterSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-additionalmastersecuritygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub additional_master_security_groups: Option<::ValueList<String>>,
        /// Property [`AdditionalSlaveSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-additionalslavesecuritygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub additional_slave_security_groups: Option<::ValueList<String>>,
        /// Property [`CoreInstanceFleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-coreinstancefleet).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub core_instance_fleet: Option<::Value<InstanceFleetConfig>>,
        /// Property [`CoreInstanceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-coreinstancegroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub core_instance_group: Option<::Value<InstanceGroupConfig>>,
        /// Property [`Ec2KeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-ec2keyname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_key_name: Option<::Value<String>>,
        /// Property [`Ec2SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-ec2subnetid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_subnet_id: Option<::Value<String>>,
        /// Property [`EmrManagedMasterSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-emrmanagedmastersecuritygroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub emr_managed_master_security_group: Option<::Value<String>>,
        /// Property [`EmrManagedSlaveSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-emrmanagedslavesecuritygroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub emr_managed_slave_security_group: Option<::Value<String>>,
        /// Property [`HadoopVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-hadoopversion).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hadoop_version: Option<::Value<String>>,
        /// Property [`MasterInstanceFleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-masterinstancefleet).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub master_instance_fleet: Option<::Value<InstanceFleetConfig>>,
        /// Property [`MasterInstanceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-masterinstancegroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub master_instance_group: Option<::Value<InstanceGroupConfig>>,
        /// Property [`Placement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-placement).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub placement: Option<::Value<PlacementType>>,
        /// Property [`ServiceAccessSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-serviceaccesssecuritygroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub service_access_security_group: Option<::Value<String>>,
        /// Property [`TerminationProtected`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html#cfn-elasticmapreduce-cluster-jobflowinstancesconfig-terminationprotected).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub termination_protected: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for JobFlowInstancesConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref additional_master_security_groups) = self.additional_master_security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalMasterSecurityGroups", additional_master_security_groups)?;
            }
            if let Some(ref additional_slave_security_groups) = self.additional_slave_security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalSlaveSecurityGroups", additional_slave_security_groups)?;
            }
            if let Some(ref core_instance_fleet) = self.core_instance_fleet {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreInstanceFleet", core_instance_fleet)?;
            }
            if let Some(ref core_instance_group) = self.core_instance_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreInstanceGroup", core_instance_group)?;
            }
            if let Some(ref ec2_key_name) = self.ec2_key_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2KeyName", ec2_key_name)?;
            }
            if let Some(ref ec2_subnet_id) = self.ec2_subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2SubnetId", ec2_subnet_id)?;
            }
            if let Some(ref emr_managed_master_security_group) = self.emr_managed_master_security_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmrManagedMasterSecurityGroup", emr_managed_master_security_group)?;
            }
            if let Some(ref emr_managed_slave_security_group) = self.emr_managed_slave_security_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmrManagedSlaveSecurityGroup", emr_managed_slave_security_group)?;
            }
            if let Some(ref hadoop_version) = self.hadoop_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HadoopVersion", hadoop_version)?;
            }
            if let Some(ref master_instance_fleet) = self.master_instance_fleet {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterInstanceFleet", master_instance_fleet)?;
            }
            if let Some(ref master_instance_group) = self.master_instance_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterInstanceGroup", master_instance_group)?;
            }
            if let Some(ref placement) = self.placement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Placement", placement)?;
            }
            if let Some(ref service_access_security_group) = self.service_access_security_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessSecurityGroup", service_access_security_group)?;
            }
            if let Some(ref termination_protected) = self.termination_protected {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminationProtected", termination_protected)?;
            }
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
                    let mut additional_master_security_groups: Option<::ValueList<String>> = None;
                    let mut additional_slave_security_groups: Option<::ValueList<String>> = None;
                    let mut core_instance_fleet: Option<::Value<InstanceFleetConfig>> = None;
                    let mut core_instance_group: Option<::Value<InstanceGroupConfig>> = None;
                    let mut ec2_key_name: Option<::Value<String>> = None;
                    let mut ec2_subnet_id: Option<::Value<String>> = None;
                    let mut emr_managed_master_security_group: Option<::Value<String>> = None;
                    let mut emr_managed_slave_security_group: Option<::Value<String>> = None;
                    let mut hadoop_version: Option<::Value<String>> = None;
                    let mut master_instance_fleet: Option<::Value<InstanceFleetConfig>> = None;
                    let mut master_instance_group: Option<::Value<InstanceGroupConfig>> = None;
                    let mut placement: Option<::Value<PlacementType>> = None;
                    let mut service_access_security_group: Option<::Value<String>> = None;
                    let mut termination_protected: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalMasterSecurityGroups" => {
                                additional_master_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdditionalSlaveSecurityGroups" => {
                                additional_slave_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoreInstanceFleet" => {
                                core_instance_fleet = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoreInstanceGroup" => {
                                core_instance_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2KeyName" => {
                                ec2_key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2SubnetId" => {
                                ec2_subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmrManagedMasterSecurityGroup" => {
                                emr_managed_master_security_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmrManagedSlaveSecurityGroup" => {
                                emr_managed_slave_security_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HadoopVersion" => {
                                hadoop_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterInstanceFleet" => {
                                master_instance_fleet = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterInstanceGroup" => {
                                master_instance_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Placement" => {
                                placement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessSecurityGroup" => {
                                service_access_security_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TerminationProtected" => {
                                termination_protected = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-metricdimension.html#cfn-elasticmapreduce-cluster-metricdimension-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-metricdimension.html#cfn-elasticmapreduce-cluster-metricdimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct PlacementType {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-placementtype.html#cfn-elasticmapreduce-cluster-placementtype-availabilityzone).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
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
                    let mut availability_zone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingAction {
        /// Property [`Market`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingaction.html#cfn-elasticmapreduce-cluster-scalingaction-market).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub market: Option<::Value<String>>,
        /// Property [`SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingaction.html#cfn-elasticmapreduce-cluster-scalingaction-simplescalingpolicyconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub simple_scaling_policy_configuration: ::Value<SimpleScalingPolicyConfiguration>,
    }

    impl ::codec::SerializeValue for ScalingAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref market) = self.market {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", market)?;
            }
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
                    let mut market: Option<::Value<String>> = None;
                    let mut simple_scaling_policy_configuration: Option<::Value<SimpleScalingPolicyConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Market" => {
                                market = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SimpleScalingPolicyConfiguration" => {
                                simple_scaling_policy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingConstraints {
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingconstraints.html#cfn-elasticmapreduce-cluster-scalingconstraints-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: ::Value<u32>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingconstraints.html#cfn-elasticmapreduce-cluster-scalingconstraints-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut max_capacity: Option<::Value<u32>> = None;
                    let mut min_capacity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingRule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html#cfn-elasticmapreduce-cluster-scalingrule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<ScalingAction>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html#cfn-elasticmapreduce-cluster-scalingrule-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html#cfn-elasticmapreduce-cluster-scalingrule-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html#cfn-elasticmapreduce-cluster-scalingrule-trigger).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger: ::Value<ScalingTrigger>,
    }

    impl ::codec::SerializeValue for ScalingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
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
                    let mut action: Option<::Value<ScalingAction>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut trigger: Option<::Value<ScalingTrigger>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trigger" => {
                                trigger = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingTrigger {
        /// Property [`CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingtrigger.html#cfn-elasticmapreduce-cluster-scalingtrigger-cloudwatchalarmdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut cloud_watch_alarm_definition: Option<::Value<CloudWatchAlarmDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchAlarmDefinition" => {
                                cloud_watch_alarm_definition = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScriptBootstrapActionConfig {
        /// Property [`Args`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scriptbootstrapactionconfig.html#cfn-elasticmapreduce-cluster-scriptbootstrapactionconfig-args).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub args: Option<::ValueList<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scriptbootstrapactionconfig.html#cfn-elasticmapreduce-cluster-scriptbootstrapactionconfig-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
    }

    impl ::codec::SerializeValue for ScriptBootstrapActionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref args) = self.args {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Args", args)?;
            }
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
                    let mut args: Option<::ValueList<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Args" => {
                                args = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property [`AdjustmentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html#cfn-elasticmapreduce-cluster-simplescalingpolicyconfiguration-adjustmenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adjustment_type: Option<::Value<String>>,
        /// Property [`CoolDown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html#cfn-elasticmapreduce-cluster-simplescalingpolicyconfiguration-cooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cool_down: Option<::Value<u32>>,
        /// Property [`ScalingAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html#cfn-elasticmapreduce-cluster-simplescalingpolicyconfiguration-scalingadjustment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scaling_adjustment: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SimpleScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adjustment_type) = self.adjustment_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", adjustment_type)?;
            }
            if let Some(ref cool_down) = self.cool_down {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoolDown", cool_down)?;
            }
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
                    let mut adjustment_type: Option<::Value<String>> = None;
                    let mut cool_down: Option<::Value<u32>> = None;
                    let mut scaling_adjustment: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdjustmentType" => {
                                adjustment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoolDown" => {
                                cool_down = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct SpotProvisioningSpecification {
        /// Property [`BlockDurationMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html#cfn-elasticmapreduce-cluster-spotprovisioningspecification-blockdurationminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_duration_minutes: Option<::Value<u32>>,
        /// Property [`TimeoutAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html#cfn-elasticmapreduce-cluster-spotprovisioningspecification-timeoutaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_action: ::Value<String>,
        /// Property [`TimeoutDurationMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html#cfn-elasticmapreduce-cluster-spotprovisioningspecification-timeoutdurationminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_duration_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SpotProvisioningSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block_duration_minutes) = self.block_duration_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDurationMinutes", block_duration_minutes)?;
            }
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
                    let mut block_duration_minutes: Option<::Value<u32>> = None;
                    let mut timeout_action: Option<::Value<String>> = None;
                    let mut timeout_duration_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockDurationMinutes" => {
                                block_duration_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutAction" => {
                                timeout_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutDurationMinutes" => {
                                timeout_duration_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct VolumeSpecification {
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html#cfn-elasticmapreduce-cluster-volumespecification-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`SizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html#cfn-elasticmapreduce-cluster-volumespecification-sizeingb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_gb: ::Value<u32>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html#cfn-elasticmapreduce-cluster-volumespecification-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for VolumeSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
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
                    let mut iops: Option<::Value<u32>> = None;
                    let mut size_in_gb: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInGB" => {
                                size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Configuration {
        /// Property [`Classification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html#cfn-elasticmapreduce-instancefleetconfig-configuration-classification).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub classification: Option<::Value<String>>,
        /// Property [`ConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html#cfn-elasticmapreduce-instancefleetconfig-configuration-configurationproperties).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configuration_properties: Option<::ValueMap<String>>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html#cfn-elasticmapreduce-instancefleetconfig-configuration-configurations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configurations: Option<::ValueList<Configuration>>,
    }

    impl ::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref classification) = self.classification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", classification)?;
            }
            if let Some(ref configuration_properties) = self.configuration_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", configuration_properties)?;
            }
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
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
                    let mut classification: Option<::Value<String>> = None;
                    let mut configuration_properties: Option<::ValueMap<String>> = None;
                    let mut configurations: Option<::ValueList<Configuration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigurationProperties" => {
                                configuration_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsBlockDeviceConfig {
        /// Property [`VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig.html#cfn-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig-volumespecification).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property [`VolumesPerInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig.html#cfn-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig-volumesperinstance).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EbsBlockDeviceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSpecification", &self.volume_specification)?;
            if let Some(ref volumes_per_instance) = self.volumes_per_instance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesPerInstance", volumes_per_instance)?;
            }
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
                    let mut volume_specification: Option<::Value<VolumeSpecification>> = None;
                    let mut volumes_per_instance: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VolumeSpecification" => {
                                volume_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumesPerInstance" => {
                                volumes_per_instance = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsConfiguration {
        /// Property [`EbsBlockDeviceConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsconfiguration.html#cfn-elasticmapreduce-instancefleetconfig-ebsconfiguration-ebsblockdeviceconfigs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property [`EbsOptimized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsconfiguration.html#cfn-elasticmapreduce-instancefleetconfig-ebsconfiguration-ebsoptimized).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs_optimized: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EbsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ebs_block_device_configs) = self.ebs_block_device_configs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsBlockDeviceConfigs", ebs_block_device_configs)?;
            }
            if let Some(ref ebs_optimized) = self.ebs_optimized {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", ebs_optimized)?;
            }
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
                    let mut ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>> = None;
                    let mut ebs_optimized: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EbsBlockDeviceConfigs" => {
                                ebs_block_device_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbsOptimized" => {
                                ebs_optimized = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property [`SpotSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancefleetprovisioningspecifications.html#cfn-elasticmapreduce-instancefleetconfig-instancefleetprovisioningspecifications-spotspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut spot_specification: Option<::Value<SpotProvisioningSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpotSpecification" => {
                                spot_specification = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct InstanceTypeConfig {
        /// Property [`BidPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfig-bidprice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bid_price: Option<::Value<String>>,
        /// Property [`BidPriceAsPercentageOfOnDemandPrice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfig-bidpriceaspercentageofondemandprice).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfig-configurations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property [`EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfig-ebsconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`WeightedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html#cfn-elasticmapreduce-instancefleetconfig-instancetypeconfig-weightedcapacity).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub weighted_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InstanceTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bid_price) = self.bid_price {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPrice", bid_price)?;
            }
            if let Some(ref bid_price_as_percentage_of_on_demand_price) = self.bid_price_as_percentage_of_on_demand_price {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPriceAsPercentageOfOnDemandPrice", bid_price_as_percentage_of_on_demand_price)?;
            }
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
            if let Some(ref ebs_configuration) = self.ebs_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsConfiguration", ebs_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref weighted_capacity) = self.weighted_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeightedCapacity", weighted_capacity)?;
            }
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
                    let mut bid_price: Option<::Value<String>> = None;
                    let mut bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>> = None;
                    let mut configurations: Option<::ValueList<Configuration>> = None;
                    let mut ebs_configuration: Option<::Value<EbsConfiguration>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut weighted_capacity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BidPrice" => {
                                bid_price = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BidPriceAsPercentageOfOnDemandPrice" => {
                                bid_price_as_percentage_of_on_demand_price = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbsConfiguration" => {
                                ebs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeightedCapacity" => {
                                weighted_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct SpotProvisioningSpecification {
        /// Property [`BlockDurationMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html#cfn-elasticmapreduce-instancefleetconfig-spotprovisioningspecification-blockdurationminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_duration_minutes: Option<::Value<u32>>,
        /// Property [`TimeoutAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html#cfn-elasticmapreduce-instancefleetconfig-spotprovisioningspecification-timeoutaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_action: ::Value<String>,
        /// Property [`TimeoutDurationMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html#cfn-elasticmapreduce-instancefleetconfig-spotprovisioningspecification-timeoutdurationminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_duration_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SpotProvisioningSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block_duration_minutes) = self.block_duration_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDurationMinutes", block_duration_minutes)?;
            }
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
                    let mut block_duration_minutes: Option<::Value<u32>> = None;
                    let mut timeout_action: Option<::Value<String>> = None;
                    let mut timeout_duration_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockDurationMinutes" => {
                                block_duration_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutAction" => {
                                timeout_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutDurationMinutes" => {
                                timeout_duration_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct VolumeSpecification {
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html#cfn-elasticmapreduce-instancefleetconfig-volumespecification-iops).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`SizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html#cfn-elasticmapreduce-instancefleetconfig-volumespecification-sizeingb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub size_in_gb: ::Value<u32>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html#cfn-elasticmapreduce-instancefleetconfig-volumespecification-volumetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for VolumeSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
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
                    let mut iops: Option<::Value<u32>> = None;
                    let mut size_in_gb: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInGB" => {
                                size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct AutoScalingPolicy {
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-autoscalingpolicy.html#cfn-elasticmapreduce-instancegroupconfig-autoscalingpolicy-constraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constraints: ::Value<ScalingConstraints>,
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-autoscalingpolicy.html#cfn-elasticmapreduce-instancegroupconfig-autoscalingpolicy-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut constraints: Option<::Value<ScalingConstraints>> = None;
                    let mut rules: Option<::ValueList<ScalingRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct CloudWatchAlarmDefinition {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: ::Value<String>,
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property [`EvaluationPeriods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-evaluationperiods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub evaluation_periods: Option<::Value<u32>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<::Value<String>>,
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: ::Value<u32>,
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: Option<::Value<String>>,
        /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-threshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold: ::Value<f64>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html#cfn-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchAlarmDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            if let Some(ref evaluation_periods) = self.evaluation_periods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationPeriods", evaluation_periods)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            if let Some(ref statistic) = self.statistic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", statistic)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
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
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut dimensions: Option<::ValueList<MetricDimension>> = None;
                    let mut evaluation_periods: Option<::Value<u32>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut namespace: Option<::Value<String>> = None;
                    let mut period: Option<::Value<u32>> = None;
                    let mut statistic: Option<::Value<String>> = None;
                    let mut threshold: Option<::Value<f64>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EvaluationPeriods" => {
                                evaluation_periods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistic" => {
                                statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Threshold" => {
                                threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Configuration {
        /// Property [`Classification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html#cfn-emr-cluster-configuration-classification).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub classification: Option<::Value<String>>,
        /// Property [`ConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html#cfn-emr-cluster-configuration-configurationproperties).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configuration_properties: Option<::ValueMap<String>>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html#cfn-emr-cluster-configuration-configurations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configurations: Option<::ValueList<Configuration>>,
    }

    impl ::codec::SerializeValue for Configuration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref classification) = self.classification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", classification)?;
            }
            if let Some(ref configuration_properties) = self.configuration_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationProperties", configuration_properties)?;
            }
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
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
                    let mut classification: Option<::Value<String>> = None;
                    let mut configuration_properties: Option<::ValueMap<String>> = None;
                    let mut configurations: Option<::ValueList<Configuration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigurationProperties" => {
                                configuration_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsBlockDeviceConfig {
        /// Property [`VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig.html#cfn-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property [`VolumesPerInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig.html#cfn-emr-ebsconfiguration-ebsblockdeviceconfig-volumesperinstance).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EbsBlockDeviceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSpecification", &self.volume_specification)?;
            if let Some(ref volumes_per_instance) = self.volumes_per_instance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesPerInstance", volumes_per_instance)?;
            }
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
                    let mut volume_specification: Option<::Value<VolumeSpecification>> = None;
                    let mut volumes_per_instance: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VolumeSpecification" => {
                                volume_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumesPerInstance" => {
                                volumes_per_instance = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsConfiguration {
        /// Property [`EbsBlockDeviceConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration.html#cfn-emr-ebsconfiguration-ebsblockdeviceconfigs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property [`EbsOptimized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration.html#cfn-emr-ebsconfiguration-ebsoptimized).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_optimized: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EbsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ebs_block_device_configs) = self.ebs_block_device_configs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsBlockDeviceConfigs", ebs_block_device_configs)?;
            }
            if let Some(ref ebs_optimized) = self.ebs_optimized {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", ebs_optimized)?;
            }
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
                    let mut ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>> = None;
                    let mut ebs_optimized: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EbsBlockDeviceConfigs" => {
                                ebs_block_device_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbsOptimized" => {
                                ebs_optimized = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-metricdimension.html#cfn-elasticmapreduce-instancegroupconfig-metricdimension-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-metricdimension.html#cfn-elasticmapreduce-instancegroupconfig-metricdimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingAction {
        /// Property [`Market`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingaction.html#cfn-elasticmapreduce-instancegroupconfig-scalingaction-market).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub market: Option<::Value<String>>,
        /// Property [`SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingaction.html#cfn-elasticmapreduce-instancegroupconfig-scalingaction-simplescalingpolicyconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub simple_scaling_policy_configuration: ::Value<SimpleScalingPolicyConfiguration>,
    }

    impl ::codec::SerializeValue for ScalingAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref market) = self.market {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Market", market)?;
            }
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
                    let mut market: Option<::Value<String>> = None;
                    let mut simple_scaling_policy_configuration: Option<::Value<SimpleScalingPolicyConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Market" => {
                                market = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SimpleScalingPolicyConfiguration" => {
                                simple_scaling_policy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingConstraints {
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingconstraints.html#cfn-elasticmapreduce-instancegroupconfig-scalingconstraints-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: ::Value<u32>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingconstraints.html#cfn-elasticmapreduce-instancegroupconfig-scalingconstraints-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut max_capacity: Option<::Value<u32>> = None;
                    let mut min_capacity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingRule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html#cfn-elasticmapreduce-instancegroupconfig-scalingrule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<ScalingAction>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html#cfn-elasticmapreduce-instancegroupconfig-scalingrule-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html#cfn-elasticmapreduce-instancegroupconfig-scalingrule-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html#cfn-elasticmapreduce-instancegroupconfig-scalingrule-trigger).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger: ::Value<ScalingTrigger>,
    }

    impl ::codec::SerializeValue for ScalingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
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
                    let mut action: Option<::Value<ScalingAction>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut trigger: Option<::Value<ScalingTrigger>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trigger" => {
                                trigger = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ScalingTrigger {
        /// Property [`CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingtrigger.html#cfn-elasticmapreduce-instancegroupconfig-scalingtrigger-cloudwatchalarmdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut cloud_watch_alarm_definition: Option<::Value<CloudWatchAlarmDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchAlarmDefinition" => {
                                cloud_watch_alarm_definition = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property [`AdjustmentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html#cfn-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration-adjustmenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adjustment_type: Option<::Value<String>>,
        /// Property [`CoolDown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html#cfn-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration-cooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cool_down: Option<::Value<u32>>,
        /// Property [`ScalingAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html#cfn-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration-scalingadjustment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scaling_adjustment: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SimpleScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adjustment_type) = self.adjustment_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", adjustment_type)?;
            }
            if let Some(ref cool_down) = self.cool_down {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoolDown", cool_down)?;
            }
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
                    let mut adjustment_type: Option<::Value<String>> = None;
                    let mut cool_down: Option<::Value<u32>> = None;
                    let mut scaling_adjustment: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdjustmentType" => {
                                adjustment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoolDown" => {
                                cool_down = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct VolumeSpecification {
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html#cfn-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`SizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html#cfn-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification-sizeingb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_gb: ::Value<u32>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html#cfn-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for VolumeSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
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
                    let mut iops: Option<::Value<u32>> = None;
                    let mut size_in_gb: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInGB" => {
                                size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct HadoopJarStepConfig {
        /// Property [`Args`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html#cfn-elasticmapreduce-step-hadoopjarstepconfig-args).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub args: Option<::ValueList<String>>,
        /// Property [`Jar`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html#cfn-elasticmapreduce-step-hadoopjarstepconfig-jar).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub jar: ::Value<String>,
        /// Property [`MainClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html#cfn-elasticmapreduce-step-hadoopjarstepconfig-mainclass).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub main_class: Option<::Value<String>>,
        /// Property [`StepProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html#cfn-elasticmapreduce-step-hadoopjarstepconfig-stepproperties).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub step_properties: Option<::ValueList<KeyValue>>,
    }

    impl ::codec::SerializeValue for HadoopJarStepConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref args) = self.args {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Args", args)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Jar", &self.jar)?;
            if let Some(ref main_class) = self.main_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MainClass", main_class)?;
            }
            if let Some(ref step_properties) = self.step_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepProperties", step_properties)?;
            }
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
                    let mut args: Option<::ValueList<String>> = None;
                    let mut jar: Option<::Value<String>> = None;
                    let mut main_class: Option<::Value<String>> = None;
                    let mut step_properties: Option<::ValueList<KeyValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Args" => {
                                args = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Jar" => {
                                jar = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MainClass" => {
                                main_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StepProperties" => {
                                step_properties = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct KeyValue {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-keyvalue.html#cfn-elasticmapreduce-step-keyvalue-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-keyvalue.html#cfn-elasticmapreduce-step-keyvalue-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KeyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
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
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
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
