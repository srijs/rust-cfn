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
    #[serde(rename="AdditionalInfo")]
    pub additional_info: ::json::Value,
    /// Property `Applications`.
    #[serde(rename="Applications")]
    pub applications: Vec<self::cluster::Application>,
    /// Property `AutoScalingRole`.
    #[serde(rename="AutoScalingRole")]
    pub auto_scaling_role: String,
    /// Property `BootstrapActions`.
    #[serde(rename="BootstrapActions")]
    pub bootstrap_actions: Vec<self::cluster::BootstrapActionConfig>,
    /// Property `Configurations`.
    #[serde(rename="Configurations")]
    pub configurations: Vec<self::cluster::Configuration>,
    /// Property `CustomAmiId`.
    #[serde(rename="CustomAmiId")]
    pub custom_ami_id: String,
    /// Property `EbsRootVolumeSize`.
    #[serde(rename="EbsRootVolumeSize")]
    pub ebs_root_volume_size: u32,
    /// Property `Instances`.
    #[serde(rename="Instances")]
    pub instances: self::cluster::JobFlowInstancesConfig,
    /// Property `JobFlowRole`.
    #[serde(rename="JobFlowRole")]
    pub job_flow_role: String,
    /// Property `LogUri`.
    #[serde(rename="LogUri")]
    pub log_uri: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `ReleaseLabel`.
    #[serde(rename="ReleaseLabel")]
    pub release_label: String,
    /// Property `ScaleDownBehavior`.
    #[serde(rename="ScaleDownBehavior")]
    pub scale_down_behavior: String,
    /// Property `SecurityConfiguration`.
    #[serde(rename="SecurityConfiguration")]
    pub security_configuration: String,
    /// Property `ServiceRole`.
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VisibleToAllUsers`.
    #[serde(rename="VisibleToAllUsers")]
    pub visible_to_all_users: bool,
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
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    /// Property `InstanceFleetType`.
    #[serde(rename="InstanceFleetType")]
    pub instance_fleet_type: String,
    /// Property `InstanceTypeConfigs`.
    #[serde(rename="InstanceTypeConfigs")]
    pub instance_type_configs: Vec<self::instance_fleet_config::InstanceTypeConfig>,
    /// Property `LaunchSpecifications`.
    #[serde(rename="LaunchSpecifications")]
    pub launch_specifications: self::instance_fleet_config::InstanceFleetProvisioningSpecifications,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `TargetOnDemandCapacity`.
    #[serde(rename="TargetOnDemandCapacity")]
    pub target_on_demand_capacity: u32,
    /// Property `TargetSpotCapacity`.
    #[serde(rename="TargetSpotCapacity")]
    pub target_spot_capacity: u32,
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
    #[serde(rename="AutoScalingPolicy")]
    pub auto_scaling_policy: self::instance_group_config::AutoScalingPolicy,
    /// Property `BidPrice`.
    #[serde(rename="BidPrice")]
    pub bid_price: String,
    /// Property `Configurations`.
    #[serde(rename="Configurations")]
    pub configurations: Vec<self::instance_group_config::Configuration>,
    /// Property `EbsConfiguration`.
    #[serde(rename="EbsConfiguration")]
    pub ebs_configuration: self::instance_group_config::EbsConfiguration,
    /// Property `InstanceCount`.
    #[serde(rename="InstanceCount")]
    pub instance_count: u32,
    /// Property `InstanceRole`.
    #[serde(rename="InstanceRole")]
    pub instance_role: String,
    /// Property `InstanceType`.
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    /// Property `JobFlowId`.
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
    /// Property `Market`.
    #[serde(rename="Market")]
    pub market: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
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
    #[serde(rename="Name")]
    pub name: String,
    /// Property `SecurityConfiguration`.
    #[serde(rename="SecurityConfiguration")]
    pub security_configuration: ::json::Value,
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
    #[serde(rename="ActionOnFailure")]
    pub action_on_failure: String,
    /// Property `HadoopJarStep`.
    #[serde(rename="HadoopJarStep")]
    pub hadoop_jar_step: self::step::HadoopJarStepConfig,
    /// Property `JobFlowId`.
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
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
        #[serde(rename="AdditionalInfo")]
        pub additional_info: ::std::collections::HashMap<String, String>,
        /// Property `Args`.
        #[serde(rename="Args")]
        pub args: Vec<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Version`.
        #[serde(rename="Version")]
        pub version: String,
    }

    /// The [`AWS::EMR::Cluster.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoscalingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingPolicy {
        /// Property `Constraints`.
        #[serde(rename="Constraints")]
        pub constraints: ScalingConstraints,
        /// Property `Rules`.
        #[serde(rename="Rules")]
        pub rules: Vec<ScalingRule>,
    }

    /// The [`AWS::EMR::Cluster.BootstrapActionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-bootstrapactionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BootstrapActionConfig {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `ScriptBootstrapAction`.
        #[serde(rename="ScriptBootstrapAction")]
        pub script_bootstrap_action: ScriptBootstrapActionConfig,
    }

    /// The [`AWS::EMR::Cluster.CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudWatchAlarmDefinition {
        /// Property `ComparisonOperator`.
        #[serde(rename="ComparisonOperator")]
        pub comparison_operator: String,
        /// Property `Dimensions`.
        #[serde(rename="Dimensions")]
        pub dimensions: Vec<MetricDimension>,
        /// Property `EvaluationPeriods`.
        #[serde(rename="EvaluationPeriods")]
        pub evaluation_periods: u32,
        /// Property `MetricName`.
        #[serde(rename="MetricName")]
        pub metric_name: String,
        /// Property `Namespace`.
        #[serde(rename="Namespace")]
        pub namespace: String,
        /// Property `Period`.
        #[serde(rename="Period")]
        pub period: u32,
        /// Property `Statistic`.
        #[serde(rename="Statistic")]
        pub statistic: String,
        /// Property `Threshold`.
        #[serde(rename="Threshold")]
        pub threshold: f64,
        /// Property `Unit`.
        #[serde(rename="Unit")]
        pub unit: String,
    }

    /// The [`AWS::EMR::Cluster.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Configuration {
        /// Property `Classification`.
        #[serde(rename="Classification")]
        pub classification: String,
        /// Property `ConfigurationProperties`.
        #[serde(rename="ConfigurationProperties")]
        pub configuration_properties: ::std::collections::HashMap<String, String>,
        /// Property `Configurations`.
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
    }

    /// The [`AWS::EMR::Cluster.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsblockdeviceconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        #[serde(rename="VolumeSpecification")]
        pub volume_specification: VolumeSpecification,
        /// Property `VolumesPerInstance`.
        #[serde(rename="VolumesPerInstance")]
        pub volumes_per_instance: u32,
    }

    /// The [`AWS::EMR::Cluster.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        #[serde(rename="EbsBlockDeviceConfigs")]
        pub ebs_block_device_configs: Vec<EbsBlockDeviceConfig>,
        /// Property `EbsOptimized`.
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
    }

    /// The [`AWS::EMR::Cluster.InstanceFleetConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceFleetConfig {
        /// Property `InstanceTypeConfigs`.
        #[serde(rename="InstanceTypeConfigs")]
        pub instance_type_configs: Vec<InstanceTypeConfig>,
        /// Property `LaunchSpecifications`.
        #[serde(rename="LaunchSpecifications")]
        pub launch_specifications: InstanceFleetProvisioningSpecifications,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `TargetOnDemandCapacity`.
        #[serde(rename="TargetOnDemandCapacity")]
        pub target_on_demand_capacity: u32,
        /// Property `TargetSpotCapacity`.
        #[serde(rename="TargetSpotCapacity")]
        pub target_spot_capacity: u32,
    }

    /// The [`AWS::EMR::Cluster.InstanceFleetProvisioningSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetprovisioningspecifications.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property `SpotSpecification`.
        #[serde(rename="SpotSpecification")]
        pub spot_specification: SpotProvisioningSpecification,
    }

    /// The [`AWS::EMR::Cluster.InstanceGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceGroupConfig {
        /// Property `AutoScalingPolicy`.
        #[serde(rename="AutoScalingPolicy")]
        pub auto_scaling_policy: AutoScalingPolicy,
        /// Property `BidPrice`.
        #[serde(rename="BidPrice")]
        pub bid_price: String,
        /// Property `Configurations`.
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
        /// Property `EbsConfiguration`.
        #[serde(rename="EbsConfiguration")]
        pub ebs_configuration: EbsConfiguration,
        /// Property `InstanceCount`.
        #[serde(rename="InstanceCount")]
        pub instance_count: u32,
        /// Property `InstanceType`.
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        /// Property `Market`.
        #[serde(rename="Market")]
        pub market: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::EMR::Cluster.InstanceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceTypeConfig {
        /// Property `BidPrice`.
        #[serde(rename="BidPrice")]
        pub bid_price: String,
        /// Property `BidPriceAsPercentageOfOnDemandPrice`.
        #[serde(rename="BidPriceAsPercentageOfOnDemandPrice")]
        pub bid_price_as_percentage_of_on_demand_price: f64,
        /// Property `Configurations`.
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
        /// Property `EbsConfiguration`.
        #[serde(rename="EbsConfiguration")]
        pub ebs_configuration: EbsConfiguration,
        /// Property `InstanceType`.
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        /// Property `WeightedCapacity`.
        #[serde(rename="WeightedCapacity")]
        pub weighted_capacity: u32,
    }

    /// The [`AWS::EMR::Cluster.JobFlowInstancesConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JobFlowInstancesConfig {
        /// Property `AdditionalMasterSecurityGroups`.
        #[serde(rename="AdditionalMasterSecurityGroups")]
        pub additional_master_security_groups: Vec<String>,
        /// Property `AdditionalSlaveSecurityGroups`.
        #[serde(rename="AdditionalSlaveSecurityGroups")]
        pub additional_slave_security_groups: Vec<String>,
        /// Property `CoreInstanceFleet`.
        #[serde(rename="CoreInstanceFleet")]
        pub core_instance_fleet: InstanceFleetConfig,
        /// Property `CoreInstanceGroup`.
        #[serde(rename="CoreInstanceGroup")]
        pub core_instance_group: InstanceGroupConfig,
        /// Property `Ec2KeyName`.
        #[serde(rename="Ec2KeyName")]
        pub ec2_key_name: String,
        /// Property `Ec2SubnetId`.
        #[serde(rename="Ec2SubnetId")]
        pub ec2_subnet_id: String,
        /// Property `EmrManagedMasterSecurityGroup`.
        #[serde(rename="EmrManagedMasterSecurityGroup")]
        pub emr_managed_master_security_group: String,
        /// Property `EmrManagedSlaveSecurityGroup`.
        #[serde(rename="EmrManagedSlaveSecurityGroup")]
        pub emr_managed_slave_security_group: String,
        /// Property `HadoopVersion`.
        #[serde(rename="HadoopVersion")]
        pub hadoop_version: String,
        /// Property `MasterInstanceFleet`.
        #[serde(rename="MasterInstanceFleet")]
        pub master_instance_fleet: InstanceFleetConfig,
        /// Property `MasterInstanceGroup`.
        #[serde(rename="MasterInstanceGroup")]
        pub master_instance_group: InstanceGroupConfig,
        /// Property `Placement`.
        #[serde(rename="Placement")]
        pub placement: PlacementType,
        /// Property `ServiceAccessSecurityGroup`.
        #[serde(rename="ServiceAccessSecurityGroup")]
        pub service_access_security_group: String,
        /// Property `TerminationProtected`.
        #[serde(rename="TerminationProtected")]
        pub termination_protected: bool,
    }

    /// The [`AWS::EMR::Cluster.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::EMR::Cluster.PlacementType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-placementtype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementType {
        /// Property `AvailabilityZone`.
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
    }

    /// The [`AWS::EMR::Cluster.ScalingAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingAction {
        /// Property `Market`.
        #[serde(rename="Market")]
        pub market: String,
        /// Property `SimpleScalingPolicyConfiguration`.
        #[serde(rename="SimpleScalingPolicyConfiguration")]
        pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
    }

    /// The [`AWS::EMR::Cluster.ScalingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingConstraints {
        /// Property `MaxCapacity`.
        #[serde(rename="MaxCapacity")]
        pub max_capacity: u32,
        /// Property `MinCapacity`.
        #[serde(rename="MinCapacity")]
        pub min_capacity: u32,
    }

    /// The [`AWS::EMR::Cluster.ScalingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingRule {
        /// Property `Action`.
        #[serde(rename="Action")]
        pub action: ScalingAction,
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Trigger`.
        #[serde(rename="Trigger")]
        pub trigger: ScalingTrigger,
    }

    /// The [`AWS::EMR::Cluster.ScalingTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingtrigger.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingTrigger {
        /// Property `CloudWatchAlarmDefinition`.
        #[serde(rename="CloudWatchAlarmDefinition")]
        pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
    }

    /// The [`AWS::EMR::Cluster.ScriptBootstrapActionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scriptbootstrapactionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScriptBootstrapActionConfig {
        /// Property `Args`.
        #[serde(rename="Args")]
        pub args: Vec<String>,
        /// Property `Path`.
        #[serde(rename="Path")]
        pub path: String,
    }

    /// The [`AWS::EMR::Cluster.SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename="AdjustmentType")]
        pub adjustment_type: String,
        /// Property `CoolDown`.
        #[serde(rename="CoolDown")]
        pub cool_down: u32,
        /// Property `ScalingAdjustment`.
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    /// The [`AWS::EMR::Cluster.SpotProvisioningSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotProvisioningSpecification {
        /// Property `BlockDurationMinutes`.
        #[serde(rename="BlockDurationMinutes")]
        pub block_duration_minutes: u32,
        /// Property `TimeoutAction`.
        #[serde(rename="TimeoutAction")]
        pub timeout_action: String,
        /// Property `TimeoutDurationMinutes`.
        #[serde(rename="TimeoutDurationMinutes")]
        pub timeout_duration_minutes: u32,
    }

    /// The [`AWS::EMR::Cluster.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `SizeInGB`.
        #[serde(rename="SizeInGB")]
        pub size_in_gb: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }
}

pub mod instance_fleet_config {
    //! Property types for the `InstanceFleetConfig` resource.

    /// The [`AWS::EMR::InstanceFleetConfig.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Configuration {
        /// Property `Classification`.
        #[serde(rename="Classification")]
        pub classification: String,
        /// Property `ConfigurationProperties`.
        #[serde(rename="ConfigurationProperties")]
        pub configuration_properties: ::std::collections::HashMap<String, String>,
        /// Property `Configurations`.
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
    }

    /// The [`AWS::EMR::InstanceFleetConfig.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        #[serde(rename="VolumeSpecification")]
        pub volume_specification: VolumeSpecification,
        /// Property `VolumesPerInstance`.
        #[serde(rename="VolumesPerInstance")]
        pub volumes_per_instance: u32,
    }

    /// The [`AWS::EMR::InstanceFleetConfig.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        #[serde(rename="EbsBlockDeviceConfigs")]
        pub ebs_block_device_configs: Vec<EbsBlockDeviceConfig>,
        /// Property `EbsOptimized`.
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
    }

    /// The [`AWS::EMR::InstanceFleetConfig.InstanceFleetProvisioningSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancefleetprovisioningspecifications.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceFleetProvisioningSpecifications {
        /// Property `SpotSpecification`.
        #[serde(rename="SpotSpecification")]
        pub spot_specification: SpotProvisioningSpecification,
    }

    /// The [`AWS::EMR::InstanceFleetConfig.InstanceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstanceTypeConfig {
        /// Property `BidPrice`.
        #[serde(rename="BidPrice")]
        pub bid_price: String,
        /// Property `BidPriceAsPercentageOfOnDemandPrice`.
        #[serde(rename="BidPriceAsPercentageOfOnDemandPrice")]
        pub bid_price_as_percentage_of_on_demand_price: f64,
        /// Property `Configurations`.
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
        /// Property `EbsConfiguration`.
        #[serde(rename="EbsConfiguration")]
        pub ebs_configuration: EbsConfiguration,
        /// Property `InstanceType`.
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        /// Property `WeightedCapacity`.
        #[serde(rename="WeightedCapacity")]
        pub weighted_capacity: u32,
    }

    /// The [`AWS::EMR::InstanceFleetConfig.SpotProvisioningSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SpotProvisioningSpecification {
        /// Property `BlockDurationMinutes`.
        #[serde(rename="BlockDurationMinutes")]
        pub block_duration_minutes: u32,
        /// Property `TimeoutAction`.
        #[serde(rename="TimeoutAction")]
        pub timeout_action: String,
        /// Property `TimeoutDurationMinutes`.
        #[serde(rename="TimeoutDurationMinutes")]
        pub timeout_duration_minutes: u32,
    }

    /// The [`AWS::EMR::InstanceFleetConfig.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `SizeInGB`.
        #[serde(rename="SizeInGB")]
        pub size_in_gb: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }
}

pub mod instance_group_config {
    //! Property types for the `InstanceGroupConfig` resource.

    /// The [`AWS::EMR::InstanceGroupConfig.AutoScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-autoscalingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingPolicy {
        /// Property `Constraints`.
        #[serde(rename="Constraints")]
        pub constraints: ScalingConstraints,
        /// Property `Rules`.
        #[serde(rename="Rules")]
        pub rules: Vec<ScalingRule>,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.CloudWatchAlarmDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudWatchAlarmDefinition {
        /// Property `ComparisonOperator`.
        #[serde(rename="ComparisonOperator")]
        pub comparison_operator: String,
        /// Property `Dimensions`.
        #[serde(rename="Dimensions")]
        pub dimensions: Vec<MetricDimension>,
        /// Property `EvaluationPeriods`.
        #[serde(rename="EvaluationPeriods")]
        pub evaluation_periods: u32,
        /// Property `MetricName`.
        #[serde(rename="MetricName")]
        pub metric_name: String,
        /// Property `Namespace`.
        #[serde(rename="Namespace")]
        pub namespace: String,
        /// Property `Period`.
        #[serde(rename="Period")]
        pub period: u32,
        /// Property `Statistic`.
        #[serde(rename="Statistic")]
        pub statistic: String,
        /// Property `Threshold`.
        #[serde(rename="Threshold")]
        pub threshold: f64,
        /// Property `Unit`.
        #[serde(rename="Unit")]
        pub unit: String,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Configuration {
        /// Property `Classification`.
        #[serde(rename="Classification")]
        pub classification: String,
        /// Property `ConfigurationProperties`.
        #[serde(rename="ConfigurationProperties")]
        pub configuration_properties: ::std::collections::HashMap<String, String>,
        /// Property `Configurations`.
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.EbsBlockDeviceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        /// Property `VolumeSpecification`.
        #[serde(rename="VolumeSpecification")]
        pub volume_specification: VolumeSpecification,
        /// Property `VolumesPerInstance`.
        #[serde(rename="VolumesPerInstance")]
        pub volumes_per_instance: u32,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.EbsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsConfiguration {
        /// Property `EbsBlockDeviceConfigs`.
        #[serde(rename="EbsBlockDeviceConfigs")]
        pub ebs_block_device_configs: Vec<EbsBlockDeviceConfig>,
        /// Property `EbsOptimized`.
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-metricdimension.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricDimension {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingaction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingAction {
        /// Property `Market`.
        #[serde(rename="Market")]
        pub market: String,
        /// Property `SimpleScalingPolicyConfiguration`.
        #[serde(rename="SimpleScalingPolicyConfiguration")]
        pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingConstraints {
        /// Property `MaxCapacity`.
        #[serde(rename="MaxCapacity")]
        pub max_capacity: u32,
        /// Property `MinCapacity`.
        #[serde(rename="MinCapacity")]
        pub min_capacity: u32,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingRule {
        /// Property `Action`.
        #[serde(rename="Action")]
        pub action: ScalingAction,
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Trigger`.
        #[serde(rename="Trigger")]
        pub trigger: ScalingTrigger,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.ScalingTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingtrigger.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScalingTrigger {
        /// Property `CloudWatchAlarmDefinition`.
        #[serde(rename="CloudWatchAlarmDefinition")]
        pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.SimpleScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SimpleScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        #[serde(rename="AdjustmentType")]
        pub adjustment_type: String,
        /// Property `CoolDown`.
        #[serde(rename="CoolDown")]
        pub cool_down: u32,
        /// Property `ScalingAdjustment`.
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    /// The [`AWS::EMR::InstanceGroupConfig.VolumeSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeSpecification {
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `SizeInGB`.
        #[serde(rename="SizeInGB")]
        pub size_in_gb: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }
}

pub mod step {
    //! Property types for the `Step` resource.

    /// The [`AWS::EMR::Step.HadoopJarStepConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-hadoopjarstepconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HadoopJarStepConfig {
        /// Property `Args`.
        #[serde(rename="Args")]
        pub args: Vec<String>,
        /// Property `Jar`.
        #[serde(rename="Jar")]
        pub jar: String,
        /// Property `MainClass`.
        #[serde(rename="MainClass")]
        pub main_class: String,
        /// Property `StepProperties`.
        #[serde(rename="StepProperties")]
        pub step_properties: Vec<KeyValue>,
    }

    /// The [`AWS::EMR::Step.KeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-step-keyvalue.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyValue {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}
