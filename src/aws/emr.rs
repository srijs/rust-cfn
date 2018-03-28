/// The [`AWS::EMR::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename="AdditionalInfo")]
    pub additional_info: String,
    #[serde(rename="Applications")]
    pub applications: Vec<()>,
    #[serde(rename="AutoScalingRole")]
    pub auto_scaling_role: String,
    #[serde(rename="BootstrapActions")]
    pub bootstrap_actions: Vec<()>,
    #[serde(rename="Configurations")]
    pub configurations: Vec<()>,
    #[serde(rename="CustomAmiId")]
    pub custom_ami_id: String,
    #[serde(rename="EbsRootVolumeSize")]
    pub ebs_root_volume_size: u32,
    #[serde(rename="Instances")]
    pub instances: (),
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
    pub tags: Vec<()>,
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
    pub instance_type_configs: Vec<()>,
    #[serde(rename="LaunchSpecifications")]
    pub launch_specifications: (),
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
    pub auto_scaling_policy: (),
    #[serde(rename="BidPrice")]
    pub bid_price: String,
    #[serde(rename="Configurations")]
    pub configurations: Vec<()>,
    #[serde(rename="EbsConfiguration")]
    pub ebs_configuration: (),
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
    pub security_configuration: String,
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
    pub hadoop_jar_step: (),
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

