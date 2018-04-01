//! Types for the `EMR` service.

/// The [`AWS::EMR::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterProperties {
    /// Property `AdditionalInfo`.
    #[serde(rename = "AdditionalInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<::Value<::json::Value>>,
    /// Property `Applications`.
    #[serde(rename = "Applications")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<::ValueList<self::cluster::Application>>,
    /// Property `AutoScalingRole`.
    #[serde(rename = "AutoScalingRole")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<::Value<String>>,
    /// Property `BootstrapActions`.
    #[serde(rename = "BootstrapActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<::ValueList<self::cluster::BootstrapActionConfig>>,
    /// Property `Configurations`.
    #[serde(rename = "Configurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<::ValueList<self::cluster::Configuration>>,
    /// Property `CustomAmiId`.
    #[serde(rename = "CustomAmiId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<::Value<String>>,
    /// Property `EbsRootVolumeSize`.
    #[serde(rename = "EbsRootVolumeSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_size: Option<::Value<u32>>,
    /// Property `Instances`.
    #[serde(rename = "Instances")]
    pub instances: ::Value<self::cluster::JobFlowInstancesConfig>,
    /// Property `JobFlowRole`.
    #[serde(rename = "JobFlowRole")]
    pub job_flow_role: ::Value<String>,
    /// Property `LogUri`.
    #[serde(rename = "LogUri")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `ReleaseLabel`.
    #[serde(rename = "ReleaseLabel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_label: Option<::Value<String>>,
    /// Property `ScaleDownBehavior`.
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<::Value<String>>,
    /// Property `SecurityConfiguration`.
    #[serde(rename = "SecurityConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<::Value<String>>,
    /// Property `ServiceRole`.
    #[serde(rename = "ServiceRole")]
    pub service_role: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VisibleToAllUsers`.
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<::Value<bool>>,
}

impl<'a> ::Resource<'a> for Cluster {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceFleetConfigProperties {
    /// Property `ClusterId`.
    #[serde(rename = "ClusterId")]
    pub cluster_id: ::Value<String>,
    /// Property `InstanceFleetType`.
    #[serde(rename = "InstanceFleetType")]
    pub instance_fleet_type: ::Value<String>,
    /// Property `InstanceTypeConfigs`.
    #[serde(rename = "InstanceTypeConfigs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_type_configs: Option<::ValueList<self::instance_fleet_config::InstanceTypeConfig>>,
    /// Property `LaunchSpecifications`.
    #[serde(rename = "LaunchSpecifications")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_specifications: Option<::Value<self::instance_fleet_config::InstanceFleetProvisioningSpecifications>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `TargetOnDemandCapacity`.
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<::Value<u32>>,
    /// Property `TargetSpotCapacity`.
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<::Value<u32>>,
}

impl<'a> ::Resource<'a> for InstanceFleetConfig {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceGroupConfigProperties {
    /// Property `AutoScalingPolicy`.
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<::Value<self::instance_group_config::AutoScalingPolicy>>,
    /// Property `BidPrice`.
    #[serde(rename = "BidPrice")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<::Value<String>>,
    /// Property `Configurations`.
    #[serde(rename = "Configurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<::ValueList<self::instance_group_config::Configuration>>,
    /// Property `EbsConfiguration`.
    #[serde(rename = "EbsConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebs_configuration: Option<::Value<self::instance_group_config::EbsConfiguration>>,
    /// Property `InstanceCount`.
    #[serde(rename = "InstanceCount")]
    pub instance_count: ::Value<u32>,
    /// Property `InstanceRole`.
    #[serde(rename = "InstanceRole")]
    pub instance_role: ::Value<String>,
    /// Property `InstanceType`.
    #[serde(rename = "InstanceType")]
    pub instance_type: ::Value<String>,
    /// Property `JobFlowId`.
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: ::Value<String>,
    /// Property `Market`.
    #[serde(rename = "Market")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for InstanceGroupConfig {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityConfigurationProperties {
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `SecurityConfiguration`.
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: ::Value<::json::Value>,
}

impl<'a> ::Resource<'a> for SecurityConfiguration {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct StepProperties {
    /// Property `ActionOnFailure`.
    #[serde(rename = "ActionOnFailure")]
    pub action_on_failure: ::Value<String>,
    /// Property `HadoopJarStep`.
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: ::Value<self::step::HadoopJarStepConfig>,
    /// Property `JobFlowId`.
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: ::Value<String>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
}

impl<'a> ::Resource<'a> for Step {
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Application {
        /// Property `AdditionalInfo`.
        #[serde(rename = "AdditionalInfo")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub additional_info: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `Args`.
        #[serde(rename = "Args")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub args: Option<::ValueList<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Version`.
        #[serde(rename = "Version")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Application);

    /// The [`AWS::EMR::Cluster.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoscalingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingPolicy {
        /// Property `Constraints`.
        #[serde(rename = "Constraints")]
        pub constraints: ::Value<ScalingConstraints>,
        /// Property `Rules`.
        #[serde(rename = "Rules")]
        pub rules: ::ValueList<ScalingRule>,
    }

    cfn_internal__inherit_codec_impls!(AutoScalingPolicy);

    /// The [`AWS::EMR::Cluster.BootstrapActionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-bootstrapactionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BootstrapActionConfig {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `ScriptBootstrapAction`.
        #[serde(rename = "ScriptBootstrapAction")]
        pub script_bootstrap_action: ::Value<ScriptBootstrapActionConfig>,
    }

    cfn_internal__inherit_codec_impls!(BootstrapActionConfig);

    /// The [`AWS::EMR::Cluster.CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudWatchAlarmDefinition {
        /// Property `ComparisonOperator`.
        #[serde(rename = "ComparisonOperator")]
        pub comparison_operator: ::Value<String>,
        /// Property `Dimensions`.
        #[serde(rename = "Dimensions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `EvaluationPeriods`.
        #[serde(rename = "EvaluationPeriods")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub evaluation_periods: Option<::Value<u32>>,
        /// Property `MetricName`.
        #[serde(rename = "MetricName")]
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        #[serde(rename = "Namespace")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<::Value<String>>,
        /// Property `Period`.
        #[serde(rename = "Period")]
        pub period: ::Value<u32>,
        /// Property `Statistic`.
        #[serde(rename = "Statistic")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub statistic: Option<::Value<String>>,
        /// Property `Threshold`.
        #[serde(rename = "Threshold")]
        pub threshold: ::Value<f64>,
        /// Property `Unit`.
        #[serde(rename = "Unit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CloudWatchAlarmDefinition);

    /// The [`AWS::EMR::Cluster.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Configuration {
        /// Property `Classification`.
        #[serde(rename = "Classification")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub classification: Option<::Value<String>>,
        /// Property `ConfigurationProperties`.
        #[serde(rename = "ConfigurationProperties")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration_properties: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `Configurations`.
        #[serde(rename = "Configurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurations: Option<::ValueList<Configuration>>,
    }

    cfn_internal__inherit_codec_impls!(Configuration);

    /// The [`AWS::EMR::Cluster.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsblockdeviceconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        #[serde(rename = "VolumeSpecification")]
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property `VolumesPerInstance`.
        #[serde(rename = "VolumesPerInstance")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(EbsBlockDeviceConfig);

    /// The [`AWS::EMR::Cluster.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        #[serde(rename = "EbsBlockDeviceConfigs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property `EbsOptimized`.
        #[serde(rename = "EbsOptimized")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_optimized: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(EbsConfiguration);

    /// The [`AWS::EMR::Cluster.InstanceFleetConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceFleetConfig {
        /// Property `InstanceTypeConfigs`.
        #[serde(rename = "InstanceTypeConfigs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance_type_configs: Option<::ValueList<InstanceTypeConfig>>,
        /// Property `LaunchSpecifications`.
        #[serde(rename = "LaunchSpecifications")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub launch_specifications: Option<::Value<InstanceFleetProvisioningSpecifications>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `TargetOnDemandCapacity`.
        #[serde(rename = "TargetOnDemandCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_on_demand_capacity: Option<::Value<u32>>,
        /// Property `TargetSpotCapacity`.
        #[serde(rename = "TargetSpotCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_spot_capacity: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(InstanceFleetConfig);

    /// The [`AWS::EMR::Cluster.InstanceFleetProvisioningSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetprovisioningspecifications.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property `SpotSpecification`.
        #[serde(rename = "SpotSpecification")]
        pub spot_specification: ::Value<SpotProvisioningSpecification>,
    }

    cfn_internal__inherit_codec_impls!(InstanceFleetProvisioningSpecifications);

    /// The [`AWS::EMR::Cluster.InstanceGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceGroupConfig {
        /// Property `AutoScalingPolicy`.
        #[serde(rename = "AutoScalingPolicy")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auto_scaling_policy: Option<::Value<AutoScalingPolicy>>,
        /// Property `BidPrice`.
        #[serde(rename = "BidPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bid_price: Option<::Value<String>>,
        /// Property `Configurations`.
        #[serde(rename = "Configurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property `EbsConfiguration`.
        #[serde(rename = "EbsConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property `InstanceCount`.
        #[serde(rename = "InstanceCount")]
        pub instance_count: ::Value<u32>,
        /// Property `InstanceType`.
        #[serde(rename = "InstanceType")]
        pub instance_type: ::Value<String>,
        /// Property `Market`.
        #[serde(rename = "Market")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub market: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(InstanceGroupConfig);

    /// The [`AWS::EMR::Cluster.InstanceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceTypeConfig {
        /// Property `BidPrice`.
        #[serde(rename = "BidPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bid_price: Option<::Value<String>>,
        /// Property `BidPriceAsPercentageOfOnDemandPrice`.
        #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>>,
        /// Property `Configurations`.
        #[serde(rename = "Configurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property `EbsConfiguration`.
        #[serde(rename = "EbsConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property `InstanceType`.
        #[serde(rename = "InstanceType")]
        pub instance_type: ::Value<String>,
        /// Property `WeightedCapacity`.
        #[serde(rename = "WeightedCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub weighted_capacity: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(InstanceTypeConfig);

    /// The [`AWS::EMR::Cluster.JobFlowInstancesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JobFlowInstancesConfig {
        /// Property `AdditionalMasterSecurityGroups`.
        #[serde(rename = "AdditionalMasterSecurityGroups")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub additional_master_security_groups: Option<::ValueList<String>>,
        /// Property `AdditionalSlaveSecurityGroups`.
        #[serde(rename = "AdditionalSlaveSecurityGroups")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub additional_slave_security_groups: Option<::ValueList<String>>,
        /// Property `CoreInstanceFleet`.
        #[serde(rename = "CoreInstanceFleet")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub core_instance_fleet: Option<::Value<InstanceFleetConfig>>,
        /// Property `CoreInstanceGroup`.
        #[serde(rename = "CoreInstanceGroup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub core_instance_group: Option<::Value<InstanceGroupConfig>>,
        /// Property `Ec2KeyName`.
        #[serde(rename = "Ec2KeyName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ec2_key_name: Option<::Value<String>>,
        /// Property `Ec2SubnetId`.
        #[serde(rename = "Ec2SubnetId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ec2_subnet_id: Option<::Value<String>>,
        /// Property `EmrManagedMasterSecurityGroup`.
        #[serde(rename = "EmrManagedMasterSecurityGroup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub emr_managed_master_security_group: Option<::Value<String>>,
        /// Property `EmrManagedSlaveSecurityGroup`.
        #[serde(rename = "EmrManagedSlaveSecurityGroup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub emr_managed_slave_security_group: Option<::Value<String>>,
        /// Property `HadoopVersion`.
        #[serde(rename = "HadoopVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hadoop_version: Option<::Value<String>>,
        /// Property `MasterInstanceFleet`.
        #[serde(rename = "MasterInstanceFleet")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub master_instance_fleet: Option<::Value<InstanceFleetConfig>>,
        /// Property `MasterInstanceGroup`.
        #[serde(rename = "MasterInstanceGroup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub master_instance_group: Option<::Value<InstanceGroupConfig>>,
        /// Property `Placement`.
        #[serde(rename = "Placement")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub placement: Option<::Value<PlacementType>>,
        /// Property `ServiceAccessSecurityGroup`.
        #[serde(rename = "ServiceAccessSecurityGroup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service_access_security_group: Option<::Value<String>>,
        /// Property `TerminationProtected`.
        #[serde(rename = "TerminationProtected")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub termination_protected: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(JobFlowInstancesConfig);

    /// The [`AWS::EMR::Cluster.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(MetricDimension);

    /// The [`AWS::EMR::Cluster.PlacementType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-placementtype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementType {
        /// Property `AvailabilityZone`.
        #[serde(rename = "AvailabilityZone")]
        pub availability_zone: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PlacementType);

    /// The [`AWS::EMR::Cluster.ScalingAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingAction {
        /// Property `Market`.
        #[serde(rename = "Market")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub market: Option<::Value<String>>,
        /// Property `SimpleScalingPolicyConfiguration`.
        #[serde(rename = "SimpleScalingPolicyConfiguration")]
        pub simple_scaling_policy_configuration: ::Value<SimpleScalingPolicyConfiguration>,
    }

    cfn_internal__inherit_codec_impls!(ScalingAction);

    /// The [`AWS::EMR::Cluster.ScalingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingConstraints {
        /// Property `MaxCapacity`.
        #[serde(rename = "MaxCapacity")]
        pub max_capacity: ::Value<u32>,
        /// Property `MinCapacity`.
        #[serde(rename = "MinCapacity")]
        pub min_capacity: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ScalingConstraints);

    /// The [`AWS::EMR::Cluster.ScalingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingRule {
        /// Property `Action`.
        #[serde(rename = "Action")]
        pub action: ::Value<ScalingAction>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Trigger`.
        #[serde(rename = "Trigger")]
        pub trigger: ::Value<ScalingTrigger>,
    }

    cfn_internal__inherit_codec_impls!(ScalingRule);

    /// The [`AWS::EMR::Cluster.ScalingTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingtrigger.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingTrigger {
        /// Property `CloudWatchAlarmDefinition`.
        #[serde(rename = "CloudWatchAlarmDefinition")]
        pub cloud_watch_alarm_definition: ::Value<CloudWatchAlarmDefinition>,
    }

    cfn_internal__inherit_codec_impls!(ScalingTrigger);

    /// The [`AWS::EMR::Cluster.ScriptBootstrapActionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scriptbootstrapactionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScriptBootstrapActionConfig {
        /// Property `Args`.
        #[serde(rename = "Args")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub args: Option<::ValueList<String>>,
        /// Property `Path`.
        #[serde(rename = "Path")]
        pub path: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ScriptBootstrapActionConfig);

    /// The [`AWS::EMR::Cluster.SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename = "AdjustmentType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub adjustment_type: Option<::Value<String>>,
        /// Property `CoolDown`.
        #[serde(rename = "CoolDown")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cool_down: Option<::Value<u32>>,
        /// Property `ScalingAdjustment`.
        #[serde(rename = "ScalingAdjustment")]
        pub scaling_adjustment: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(SimpleScalingPolicyConfiguration);

    /// The [`AWS::EMR::Cluster.SpotProvisioningSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotProvisioningSpecification {
        /// Property `BlockDurationMinutes`.
        #[serde(rename = "BlockDurationMinutes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block_duration_minutes: Option<::Value<u32>>,
        /// Property `TimeoutAction`.
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: ::Value<String>,
        /// Property `TimeoutDurationMinutes`.
        #[serde(rename = "TimeoutDurationMinutes")]
        pub timeout_duration_minutes: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(SpotProvisioningSpecification);

    /// The [`AWS::EMR::Cluster.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SizeInGB`.
        #[serde(rename = "SizeInGB")]
        pub size_in_gb: ::Value<u32>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        pub volume_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(VolumeSpecification);
}

pub mod instance_fleet_config {
    //! Property types for the `InstanceFleetConfig` resource.

    /// The [`AWS::EMR::InstanceFleetConfig.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Configuration {
        /// Property `Classification`.
        #[serde(rename = "Classification")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub classification: Option<::Value<String>>,
        /// Property `ConfigurationProperties`.
        #[serde(rename = "ConfigurationProperties")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration_properties: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `Configurations`.
        #[serde(rename = "Configurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurations: Option<::ValueList<Configuration>>,
    }

    cfn_internal__inherit_codec_impls!(Configuration);

    /// The [`AWS::EMR::InstanceFleetConfig.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        #[serde(rename = "VolumeSpecification")]
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property `VolumesPerInstance`.
        #[serde(rename = "VolumesPerInstance")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(EbsBlockDeviceConfig);

    /// The [`AWS::EMR::InstanceFleetConfig.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        #[serde(rename = "EbsBlockDeviceConfigs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property `EbsOptimized`.
        #[serde(rename = "EbsOptimized")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_optimized: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(EbsConfiguration);

    /// The [`AWS::EMR::InstanceFleetConfig.InstanceFleetProvisioningSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancefleetprovisioningspecifications.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property `SpotSpecification`.
        #[serde(rename = "SpotSpecification")]
        pub spot_specification: ::Value<SpotProvisioningSpecification>,
    }

    cfn_internal__inherit_codec_impls!(InstanceFleetProvisioningSpecifications);

    /// The [`AWS::EMR::InstanceFleetConfig.InstanceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceTypeConfig {
        /// Property `BidPrice`.
        #[serde(rename = "BidPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bid_price: Option<::Value<String>>,
        /// Property `BidPriceAsPercentageOfOnDemandPrice`.
        #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bid_price_as_percentage_of_on_demand_price: Option<::Value<f64>>,
        /// Property `Configurations`.
        #[serde(rename = "Configurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurations: Option<::ValueList<Configuration>>,
        /// Property `EbsConfiguration`.
        #[serde(rename = "EbsConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_configuration: Option<::Value<EbsConfiguration>>,
        /// Property `InstanceType`.
        #[serde(rename = "InstanceType")]
        pub instance_type: ::Value<String>,
        /// Property `WeightedCapacity`.
        #[serde(rename = "WeightedCapacity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub weighted_capacity: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(InstanceTypeConfig);

    /// The [`AWS::EMR::InstanceFleetConfig.SpotProvisioningSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotProvisioningSpecification {
        /// Property `BlockDurationMinutes`.
        #[serde(rename = "BlockDurationMinutes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block_duration_minutes: Option<::Value<u32>>,
        /// Property `TimeoutAction`.
        #[serde(rename = "TimeoutAction")]
        pub timeout_action: ::Value<String>,
        /// Property `TimeoutDurationMinutes`.
        #[serde(rename = "TimeoutDurationMinutes")]
        pub timeout_duration_minutes: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(SpotProvisioningSpecification);

    /// The [`AWS::EMR::InstanceFleetConfig.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SizeInGB`.
        #[serde(rename = "SizeInGB")]
        pub size_in_gb: ::Value<u32>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        pub volume_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(VolumeSpecification);
}

pub mod instance_group_config {
    //! Property types for the `InstanceGroupConfig` resource.

    /// The [`AWS::EMR::InstanceGroupConfig.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-autoscalingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingPolicy {
        /// Property `Constraints`.
        #[serde(rename = "Constraints")]
        pub constraints: ::Value<ScalingConstraints>,
        /// Property `Rules`.
        #[serde(rename = "Rules")]
        pub rules: ::ValueList<ScalingRule>,
    }

    cfn_internal__inherit_codec_impls!(AutoScalingPolicy);

    /// The [`AWS::EMR::InstanceGroupConfig.CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudWatchAlarmDefinition {
        /// Property `ComparisonOperator`.
        #[serde(rename = "ComparisonOperator")]
        pub comparison_operator: ::Value<String>,
        /// Property `Dimensions`.
        #[serde(rename = "Dimensions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `EvaluationPeriods`.
        #[serde(rename = "EvaluationPeriods")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub evaluation_periods: Option<::Value<u32>>,
        /// Property `MetricName`.
        #[serde(rename = "MetricName")]
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        #[serde(rename = "Namespace")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<::Value<String>>,
        /// Property `Period`.
        #[serde(rename = "Period")]
        pub period: ::Value<u32>,
        /// Property `Statistic`.
        #[serde(rename = "Statistic")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub statistic: Option<::Value<String>>,
        /// Property `Threshold`.
        #[serde(rename = "Threshold")]
        pub threshold: ::Value<f64>,
        /// Property `Unit`.
        #[serde(rename = "Unit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CloudWatchAlarmDefinition);

    /// The [`AWS::EMR::InstanceGroupConfig.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Configuration {
        /// Property `Classification`.
        #[serde(rename = "Classification")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub classification: Option<::Value<String>>,
        /// Property `ConfigurationProperties`.
        #[serde(rename = "ConfigurationProperties")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration_properties: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `Configurations`.
        #[serde(rename = "Configurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurations: Option<::ValueList<Configuration>>,
    }

    cfn_internal__inherit_codec_impls!(Configuration);

    /// The [`AWS::EMR::InstanceGroupConfig.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        #[serde(rename = "VolumeSpecification")]
        pub volume_specification: ::Value<VolumeSpecification>,
        /// Property `VolumesPerInstance`.
        #[serde(rename = "VolumesPerInstance")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volumes_per_instance: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(EbsBlockDeviceConfig);

    /// The [`AWS::EMR::InstanceGroupConfig.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        #[serde(rename = "EbsBlockDeviceConfigs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_block_device_configs: Option<::ValueList<EbsBlockDeviceConfig>>,
        /// Property `EbsOptimized`.
        #[serde(rename = "EbsOptimized")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs_optimized: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(EbsConfiguration);

    /// The [`AWS::EMR::InstanceGroupConfig.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(MetricDimension);

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingAction {
        /// Property `Market`.
        #[serde(rename = "Market")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub market: Option<::Value<String>>,
        /// Property `SimpleScalingPolicyConfiguration`.
        #[serde(rename = "SimpleScalingPolicyConfiguration")]
        pub simple_scaling_policy_configuration: ::Value<SimpleScalingPolicyConfiguration>,
    }

    cfn_internal__inherit_codec_impls!(ScalingAction);

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingConstraints {
        /// Property `MaxCapacity`.
        #[serde(rename = "MaxCapacity")]
        pub max_capacity: ::Value<u32>,
        /// Property `MinCapacity`.
        #[serde(rename = "MinCapacity")]
        pub min_capacity: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ScalingConstraints);

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingRule {
        /// Property `Action`.
        #[serde(rename = "Action")]
        pub action: ::Value<ScalingAction>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Trigger`.
        #[serde(rename = "Trigger")]
        pub trigger: ::Value<ScalingTrigger>,
    }

    cfn_internal__inherit_codec_impls!(ScalingRule);

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingtrigger.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingTrigger {
        /// Property `CloudWatchAlarmDefinition`.
        #[serde(rename = "CloudWatchAlarmDefinition")]
        pub cloud_watch_alarm_definition: ::Value<CloudWatchAlarmDefinition>,
    }

    cfn_internal__inherit_codec_impls!(ScalingTrigger);

    /// The [`AWS::EMR::InstanceGroupConfig.SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename = "AdjustmentType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub adjustment_type: Option<::Value<String>>,
        /// Property `CoolDown`.
        #[serde(rename = "CoolDown")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cool_down: Option<::Value<u32>>,
        /// Property `ScalingAdjustment`.
        #[serde(rename = "ScalingAdjustment")]
        pub scaling_adjustment: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(SimpleScalingPolicyConfiguration);

    /// The [`AWS::EMR::InstanceGroupConfig.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SizeInGB`.
        #[serde(rename = "SizeInGB")]
        pub size_in_gb: ::Value<u32>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        pub volume_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(VolumeSpecification);
}

pub mod step {
    //! Property types for the `Step` resource.

    /// The [`AWS::EMR::Step.HadoopJarStepConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HadoopJarStepConfig {
        /// Property `Args`.
        #[serde(rename = "Args")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub args: Option<::ValueList<String>>,
        /// Property `Jar`.
        #[serde(rename = "Jar")]
        pub jar: ::Value<String>,
        /// Property `MainClass`.
        #[serde(rename = "MainClass")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub main_class: Option<::Value<String>>,
        /// Property `StepProperties`.
        #[serde(rename = "StepProperties")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub step_properties: Option<::ValueList<KeyValue>>,
    }

    cfn_internal__inherit_codec_impls!(HadoopJarStepConfig);

    /// The [`AWS::EMR::Step.KeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-keyvalue.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyValue {
        /// Property `Key`.
        #[serde(rename = "Key")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(KeyValue);
}
