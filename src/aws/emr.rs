/// The [`AWS::EMR::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename="AdditionalInfo")]
    pub additional_info: ::json::Value,
    #[serde(rename="Applications")]
    pub applications: Vec<self::cluster::Application>,
    #[serde(rename="AutoScalingRole")]
    pub auto_scaling_role: String,
    #[serde(rename="BootstrapActions")]
    pub bootstrap_actions: Vec<self::cluster::BootstrapActionConfig>,
    #[serde(rename="Configurations")]
    pub configurations: Vec<self::cluster::Configuration>,
    #[serde(rename="CustomAmiId")]
    pub custom_ami_id: String,
    #[serde(rename="EbsRootVolumeSize")]
    pub ebs_root_volume_size: u32,
    #[serde(rename="Instances")]
    pub instances: self::cluster::JobFlowInstancesConfig,
    #[serde(rename="JobFlowRole")]
    pub job_flow_role: String,
    #[serde(rename="LogUri")]
    pub log_uri: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ReleaseLabel")]
    pub release_label: String,
    #[serde(rename="ScaleDownBehavior")]
    pub scale_down_behavior: String,
    #[serde(rename="SecurityConfiguration")]
    pub security_configuration: String,
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::EMR::InstanceFleetConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html) resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceFleetConfig {
    properties: InstanceFleetConfigProperties
}

/// Properties for the `InstanceFleetConfig` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceFleetConfigProperties {
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[serde(rename="InstanceFleetType")]
    pub instance_fleet_type: String,
    #[serde(rename="InstanceTypeConfigs")]
    pub instance_type_configs: Vec<self::instance_fleet_config::InstanceTypeConfig>,
    #[serde(rename="LaunchSpecifications")]
    pub launch_specifications: self::instance_fleet_config::InstanceFleetProvisioningSpecifications,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="TargetOnDemandCapacity")]
    pub target_on_demand_capacity: u32,
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

impl From<InstanceFleetConfigProperties> for InstanceFleetConfig {
    fn from(properties: InstanceFleetConfigProperties) -> InstanceFleetConfig {
        InstanceFleetConfig { properties }
    }
}

/// The [`AWS::EMR::InstanceGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html) resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceGroupConfig {
    properties: InstanceGroupConfigProperties
}

/// Properties for the `InstanceGroupConfig` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceGroupConfigProperties {
    #[serde(rename="AutoScalingPolicy")]
    pub auto_scaling_policy: self::instance_group_config::AutoScalingPolicy,
    #[serde(rename="BidPrice")]
    pub bid_price: String,
    #[serde(rename="Configurations")]
    pub configurations: Vec<self::instance_group_config::Configuration>,
    #[serde(rename="EbsConfiguration")]
    pub ebs_configuration: self::instance_group_config::EbsConfiguration,
    #[serde(rename="InstanceCount")]
    pub instance_count: u32,
    #[serde(rename="InstanceRole")]
    pub instance_role: String,
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
    #[serde(rename="Market")]
    pub market: String,
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

impl From<InstanceGroupConfigProperties> for InstanceGroupConfig {
    fn from(properties: InstanceGroupConfigProperties) -> InstanceGroupConfig {
        InstanceGroupConfig { properties }
    }
}

/// The [`AWS::EMR::SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-securityconfiguration.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityConfiguration {
    properties: SecurityConfigurationProperties
}

/// Properties for the `SecurityConfiguration` resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityConfigurationProperties {
    #[serde(rename="Name")]
    pub name: String,
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

impl From<SecurityConfigurationProperties> for SecurityConfiguration {
    fn from(properties: SecurityConfigurationProperties) -> SecurityConfiguration {
        SecurityConfiguration { properties }
    }
}

/// The [`AWS::EMR::Step`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Step {
    properties: StepProperties
}

/// Properties for the `Step` resource.
#[derive(Serialize, Deserialize)]
pub struct StepProperties {
    #[serde(rename="ActionOnFailure")]
    pub action_on_failure: String,
    #[serde(rename="HadoopJarStep")]
    pub hadoop_jar_step: self::step::HadoopJarStepConfig,
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
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

impl From<StepProperties> for Step {
    fn from(properties: StepProperties) -> Step {
        Step { properties }
    }
}

pub mod cluster {
    #[derive(Serialize, Deserialize)]
    pub struct Application {
        #[serde(rename="AdditionalInfo")]
        pub additional_info: ::std::collections::HashMap<String, String>,
        #[serde(rename="Args")]
        pub args: Vec<String>,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Version")]
        pub version: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AutoScalingPolicy {
        #[serde(rename="Constraints")]
        pub constraints: ScalingConstraints,
        #[serde(rename="Rules")]
        pub rules: Vec<ScalingRule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct BootstrapActionConfig {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="ScriptBootstrapAction")]
        pub script_bootstrap_action: ScriptBootstrapActionConfig,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CloudWatchAlarmDefinition {
        #[serde(rename="ComparisonOperator")]
        pub comparison_operator: String,
        #[serde(rename="Dimensions")]
        pub dimensions: Vec<MetricDimension>,
        #[serde(rename="EvaluationPeriods")]
        pub evaluation_periods: u32,
        #[serde(rename="MetricName")]
        pub metric_name: String,
        #[serde(rename="Namespace")]
        pub namespace: String,
        #[serde(rename="Period")]
        pub period: u32,
        #[serde(rename="Statistic")]
        pub statistic: String,
        #[serde(rename="Threshold")]
        pub threshold: f64,
        #[serde(rename="Unit")]
        pub unit: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Configuration {
        #[serde(rename="Classification")]
        pub classification: String,
        #[serde(rename="ConfigurationProperties")]
        pub configuration_properties: ::std::collections::HashMap<String, String>,
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        #[serde(rename="VolumeSpecification")]
        pub volume_specification: VolumeSpecification,
        #[serde(rename="VolumesPerInstance")]
        pub volumes_per_instance: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsConfiguration {
        #[serde(rename="EbsBlockDeviceConfigs")]
        pub ebs_block_device_configs: Vec<EbsBlockDeviceConfig>,
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InstanceFleetConfig {
        #[serde(rename="InstanceTypeConfigs")]
        pub instance_type_configs: Vec<InstanceTypeConfig>,
        #[serde(rename="LaunchSpecifications")]
        pub launch_specifications: InstanceFleetProvisioningSpecifications,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="TargetOnDemandCapacity")]
        pub target_on_demand_capacity: u32,
        #[serde(rename="TargetSpotCapacity")]
        pub target_spot_capacity: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InstanceFleetProvisioningSpecifications {
        #[serde(rename="SpotSpecification")]
        pub spot_specification: SpotProvisioningSpecification,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InstanceGroupConfig {
        #[serde(rename="AutoScalingPolicy")]
        pub auto_scaling_policy: AutoScalingPolicy,
        #[serde(rename="BidPrice")]
        pub bid_price: String,
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
        #[serde(rename="EbsConfiguration")]
        pub ebs_configuration: EbsConfiguration,
        #[serde(rename="InstanceCount")]
        pub instance_count: u32,
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        #[serde(rename="Market")]
        pub market: String,
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InstanceTypeConfig {
        #[serde(rename="BidPrice")]
        pub bid_price: String,
        #[serde(rename="BidPriceAsPercentageOfOnDemandPrice")]
        pub bid_price_as_percentage_of_on_demand_price: f64,
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
        #[serde(rename="EbsConfiguration")]
        pub ebs_configuration: EbsConfiguration,
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        #[serde(rename="WeightedCapacity")]
        pub weighted_capacity: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JobFlowInstancesConfig {
        #[serde(rename="AdditionalMasterSecurityGroups")]
        pub additional_master_security_groups: Vec<String>,
        #[serde(rename="AdditionalSlaveSecurityGroups")]
        pub additional_slave_security_groups: Vec<String>,
        #[serde(rename="CoreInstanceFleet")]
        pub core_instance_fleet: InstanceFleetConfig,
        #[serde(rename="CoreInstanceGroup")]
        pub core_instance_group: InstanceGroupConfig,
        #[serde(rename="Ec2KeyName")]
        pub ec2_key_name: String,
        #[serde(rename="Ec2SubnetId")]
        pub ec2_subnet_id: String,
        #[serde(rename="EmrManagedMasterSecurityGroup")]
        pub emr_managed_master_security_group: String,
        #[serde(rename="EmrManagedSlaveSecurityGroup")]
        pub emr_managed_slave_security_group: String,
        #[serde(rename="HadoopVersion")]
        pub hadoop_version: String,
        #[serde(rename="MasterInstanceFleet")]
        pub master_instance_fleet: InstanceFleetConfig,
        #[serde(rename="MasterInstanceGroup")]
        pub master_instance_group: InstanceGroupConfig,
        #[serde(rename="Placement")]
        pub placement: PlacementType,
        #[serde(rename="ServiceAccessSecurityGroup")]
        pub service_access_security_group: String,
        #[serde(rename="TerminationProtected")]
        pub termination_protected: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetricDimension {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PlacementType {
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingAction {
        #[serde(rename="Market")]
        pub market: String,
        #[serde(rename="SimpleScalingPolicyConfiguration")]
        pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingConstraints {
        #[serde(rename="MaxCapacity")]
        pub max_capacity: u32,
        #[serde(rename="MinCapacity")]
        pub min_capacity: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingRule {
        #[serde(rename="Action")]
        pub action: ScalingAction,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Trigger")]
        pub trigger: ScalingTrigger,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingTrigger {
        #[serde(rename="CloudWatchAlarmDefinition")]
        pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScriptBootstrapActionConfig {
        #[serde(rename="Args")]
        pub args: Vec<String>,
        #[serde(rename="Path")]
        pub path: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SimpleScalingPolicyConfiguration {
        #[serde(rename="AdjustmentType")]
        pub adjustment_type: String,
        #[serde(rename="CoolDown")]
        pub cool_down: u32,
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SpotProvisioningSpecification {
        #[serde(rename="BlockDurationMinutes")]
        pub block_duration_minutes: u32,
        #[serde(rename="TimeoutAction")]
        pub timeout_action: String,
        #[serde(rename="TimeoutDurationMinutes")]
        pub timeout_duration_minutes: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VolumeSpecification {
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SizeInGB")]
        pub size_in_gb: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

}

pub mod instance_fleet_config {
    #[derive(Serialize, Deserialize)]
    pub struct Configuration {
        #[serde(rename="Classification")]
        pub classification: String,
        #[serde(rename="ConfigurationProperties")]
        pub configuration_properties: ::std::collections::HashMap<String, String>,
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        #[serde(rename="VolumeSpecification")]
        pub volume_specification: VolumeSpecification,
        #[serde(rename="VolumesPerInstance")]
        pub volumes_per_instance: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsConfiguration {
        #[serde(rename="EbsBlockDeviceConfigs")]
        pub ebs_block_device_configs: Vec<EbsBlockDeviceConfig>,
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InstanceFleetProvisioningSpecifications {
        #[serde(rename="SpotSpecification")]
        pub spot_specification: SpotProvisioningSpecification,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InstanceTypeConfig {
        #[serde(rename="BidPrice")]
        pub bid_price: String,
        #[serde(rename="BidPriceAsPercentageOfOnDemandPrice")]
        pub bid_price_as_percentage_of_on_demand_price: f64,
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
        #[serde(rename="EbsConfiguration")]
        pub ebs_configuration: EbsConfiguration,
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        #[serde(rename="WeightedCapacity")]
        pub weighted_capacity: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SpotProvisioningSpecification {
        #[serde(rename="BlockDurationMinutes")]
        pub block_duration_minutes: u32,
        #[serde(rename="TimeoutAction")]
        pub timeout_action: String,
        #[serde(rename="TimeoutDurationMinutes")]
        pub timeout_duration_minutes: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VolumeSpecification {
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SizeInGB")]
        pub size_in_gb: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

}

pub mod instance_group_config {
    #[derive(Serialize, Deserialize)]
    pub struct AutoScalingPolicy {
        #[serde(rename="Constraints")]
        pub constraints: ScalingConstraints,
        #[serde(rename="Rules")]
        pub rules: Vec<ScalingRule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CloudWatchAlarmDefinition {
        #[serde(rename="ComparisonOperator")]
        pub comparison_operator: String,
        #[serde(rename="Dimensions")]
        pub dimensions: Vec<MetricDimension>,
        #[serde(rename="EvaluationPeriods")]
        pub evaluation_periods: u32,
        #[serde(rename="MetricName")]
        pub metric_name: String,
        #[serde(rename="Namespace")]
        pub namespace: String,
        #[serde(rename="Period")]
        pub period: u32,
        #[serde(rename="Statistic")]
        pub statistic: String,
        #[serde(rename="Threshold")]
        pub threshold: f64,
        #[serde(rename="Unit")]
        pub unit: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Configuration {
        #[serde(rename="Classification")]
        pub classification: String,
        #[serde(rename="ConfigurationProperties")]
        pub configuration_properties: ::std::collections::HashMap<String, String>,
        #[serde(rename="Configurations")]
        pub configurations: Vec<Configuration>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsBlockDeviceConfig {
        #[serde(rename="VolumeSpecification")]
        pub volume_specification: VolumeSpecification,
        #[serde(rename="VolumesPerInstance")]
        pub volumes_per_instance: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsConfiguration {
        #[serde(rename="EbsBlockDeviceConfigs")]
        pub ebs_block_device_configs: Vec<EbsBlockDeviceConfig>,
        #[serde(rename="EbsOptimized")]
        pub ebs_optimized: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetricDimension {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingAction {
        #[serde(rename="Market")]
        pub market: String,
        #[serde(rename="SimpleScalingPolicyConfiguration")]
        pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingConstraints {
        #[serde(rename="MaxCapacity")]
        pub max_capacity: u32,
        #[serde(rename="MinCapacity")]
        pub min_capacity: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingRule {
        #[serde(rename="Action")]
        pub action: ScalingAction,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Trigger")]
        pub trigger: ScalingTrigger,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ScalingTrigger {
        #[serde(rename="CloudWatchAlarmDefinition")]
        pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SimpleScalingPolicyConfiguration {
        #[serde(rename="AdjustmentType")]
        pub adjustment_type: String,
        #[serde(rename="CoolDown")]
        pub cool_down: u32,
        #[serde(rename="ScalingAdjustment")]
        pub scaling_adjustment: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VolumeSpecification {
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SizeInGB")]
        pub size_in_gb: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

}

pub mod step {
    #[derive(Serialize, Deserialize)]
    pub struct HadoopJarStepConfig {
        #[serde(rename="Args")]
        pub args: Vec<String>,
        #[serde(rename="Jar")]
        pub jar: String,
        #[serde(rename="MainClass")]
        pub main_class: String,
        #[serde(rename="StepProperties")]
        pub step_properties: Vec<KeyValue>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KeyValue {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

