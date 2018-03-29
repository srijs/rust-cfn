/// The [`AWS::OpsWorks::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html) resource.
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Serialize, Deserialize)]
pub struct AppProperties {
    #[serde(rename="AppSource")]
    pub app_source: self::app::Source,
    #[serde(rename="Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    #[serde(rename="DataSources")]
    pub data_sources: Vec<self::app::DataSource>,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Domains")]
    pub domains: Vec<String>,
    #[serde(rename="EnableSsl")]
    pub enable_ssl: bool,
    #[serde(rename="Environment")]
    pub environment: Vec<self::app::EnvironmentVariable>,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Shortname")]
    pub shortname: String,
    #[serde(rename="SslConfiguration")]
    pub ssl_configuration: self::app::SslConfiguration,
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for App {
    type Properties = AppProperties;
    const TYPE: &'static str = "AWS::OpsWorks::App";
    fn properties(&self) -> &AppProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppProperties {
        &mut self.properties
    }
}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::OpsWorks::ElasticLoadBalancerAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html) resource.
pub struct ElasticLoadBalancerAttachment {
    properties: ElasticLoadBalancerAttachmentProperties
}

/// Properties for the `ElasticLoadBalancerAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct ElasticLoadBalancerAttachmentProperties {
    #[serde(rename="ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    #[serde(rename="LayerId")]
    pub layer_id: String,
}

impl<'a> ::Resource<'a> for ElasticLoadBalancerAttachment {
    type Properties = ElasticLoadBalancerAttachmentProperties;
    const TYPE: &'static str = "AWS::OpsWorks::ElasticLoadBalancerAttachment";
    fn properties(&self) -> &ElasticLoadBalancerAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ElasticLoadBalancerAttachmentProperties {
        &mut self.properties
    }
}

impl From<ElasticLoadBalancerAttachmentProperties> for ElasticLoadBalancerAttachment {
    fn from(properties: ElasticLoadBalancerAttachmentProperties) -> ElasticLoadBalancerAttachment {
        ElasticLoadBalancerAttachment { properties }
    }
}

/// The [`AWS::OpsWorks::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html) resource.
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceProperties {
    #[serde(rename="AgentVersion")]
    pub agent_version: String,
    #[serde(rename="AmiId")]
    pub ami_id: String,
    #[serde(rename="Architecture")]
    pub architecture: String,
    #[serde(rename="AutoScalingType")]
    pub auto_scaling_type: String,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: Vec<self::instance::BlockDeviceMapping>,
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: bool,
    #[serde(rename="ElasticIps")]
    pub elastic_ips: Vec<String>,
    #[serde(rename="Hostname")]
    pub hostname: String,
    #[serde(rename="InstallUpdatesOnBoot")]
    pub install_updates_on_boot: bool,
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[serde(rename="LayerIds")]
    pub layer_ids: Vec<String>,
    #[serde(rename="Os")]
    pub os: String,
    #[serde(rename="RootDeviceType")]
    pub root_device_type: String,
    #[serde(rename="SshKeyName")]
    pub ssh_key_name: String,
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
    #[serde(rename="Tenancy")]
    pub tenancy: String,
    #[serde(rename="TimeBasedAutoScaling")]
    pub time_based_auto_scaling: self::instance::TimeBasedAutoScaling,
    #[serde(rename="VirtualizationType")]
    pub virtualization_type: String,
    #[serde(rename="Volumes")]
    pub volumes: Vec<String>,
}

impl<'a> ::Resource<'a> for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::OpsWorks::Layer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html) resource.
pub struct Layer {
    properties: LayerProperties
}

/// Properties for the `Layer` resource.
#[derive(Serialize, Deserialize)]
pub struct LayerProperties {
    #[serde(rename="Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    #[serde(rename="AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: bool,
    #[serde(rename="AutoAssignPublicIps")]
    pub auto_assign_public_ips: bool,
    #[serde(rename="CustomInstanceProfileArn")]
    pub custom_instance_profile_arn: String,
    #[serde(rename="CustomJson")]
    pub custom_json: ::json::Value,
    #[serde(rename="CustomRecipes")]
    pub custom_recipes: self::layer::Recipes,
    #[serde(rename="CustomSecurityGroupIds")]
    pub custom_security_group_ids: Vec<String>,
    #[serde(rename="EnableAutoHealing")]
    pub enable_auto_healing: bool,
    #[serde(rename="InstallUpdatesOnBoot")]
    pub install_updates_on_boot: bool,
    #[serde(rename="LifecycleEventConfiguration")]
    pub lifecycle_event_configuration: self::layer::LifecycleEventConfiguration,
    #[serde(rename="LoadBasedAutoScaling")]
    pub load_based_auto_scaling: self::layer::LoadBasedAutoScaling,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Packages")]
    pub packages: Vec<String>,
    #[serde(rename="Shortname")]
    pub shortname: String,
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Type")]
    pub type_: String,
    #[serde(rename="UseEbsOptimizedInstances")]
    pub use_ebs_optimized_instances: bool,
    #[serde(rename="VolumeConfigurations")]
    pub volume_configurations: Vec<self::layer::VolumeConfiguration>,
}

impl<'a> ::Resource<'a> for Layer {
    type Properties = LayerProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Layer";
    fn properties(&self) -> &LayerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LayerProperties {
        &mut self.properties
    }
}

impl From<LayerProperties> for Layer {
    fn from(properties: LayerProperties) -> Layer {
        Layer { properties }
    }
}

/// The [`AWS::OpsWorks::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html) resource.
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Serialize, Deserialize)]
pub struct StackProperties {
    #[serde(rename="AgentVersion")]
    pub agent_version: String,
    #[serde(rename="Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    #[serde(rename="ChefConfiguration")]
    pub chef_configuration: self::stack::ChefConfiguration,
    #[serde(rename="CloneAppIds")]
    pub clone_app_ids: Vec<String>,
    #[serde(rename="ClonePermissions")]
    pub clone_permissions: bool,
    #[serde(rename="ConfigurationManager")]
    pub configuration_manager: self::stack::StackConfigurationManager,
    #[serde(rename="CustomCookbooksSource")]
    pub custom_cookbooks_source: self::stack::Source,
    #[serde(rename="CustomJson")]
    pub custom_json: ::json::Value,
    #[serde(rename="DefaultAvailabilityZone")]
    pub default_availability_zone: String,
    #[serde(rename="DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,
    #[serde(rename="DefaultOs")]
    pub default_os: String,
    #[serde(rename="DefaultRootDeviceType")]
    pub default_root_device_type: String,
    #[serde(rename="DefaultSshKeyName")]
    pub default_ssh_key_name: String,
    #[serde(rename="DefaultSubnetId")]
    pub default_subnet_id: String,
    #[serde(rename="EcsClusterArn")]
    pub ecs_cluster_arn: String,
    #[serde(rename="ElasticIps")]
    pub elastic_ips: Vec<self::stack::ElasticIp>,
    #[serde(rename="HostnameTheme")]
    pub hostname_theme: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RdsDbInstances")]
    pub rds_db_instances: Vec<self::stack::RdsDbInstance>,
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[serde(rename="SourceStackId")]
    pub source_stack_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="UseCustomCookbooks")]
    pub use_custom_cookbooks: bool,
    #[serde(rename="UseOpsworksSecurityGroups")]
    pub use_opsworks_security_groups: bool,
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for Stack {
    type Properties = StackProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Stack";
    fn properties(&self) -> &StackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackProperties {
        &mut self.properties
    }
}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::OpsWorks::UserProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html) resource.
pub struct UserProfile {
    properties: UserProfileProperties
}

/// Properties for the `UserProfile` resource.
#[derive(Serialize, Deserialize)]
pub struct UserProfileProperties {
    #[serde(rename="AllowSelfManagement")]
    pub allow_self_management: bool,
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
    #[serde(rename="SshPublicKey")]
    pub ssh_public_key: String,
    #[serde(rename="SshUsername")]
    pub ssh_username: String,
}

impl<'a> ::Resource<'a> for UserProfile {
    type Properties = UserProfileProperties;
    const TYPE: &'static str = "AWS::OpsWorks::UserProfile";
    fn properties(&self) -> &UserProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProfileProperties {
        &mut self.properties
    }
}

impl From<UserProfileProperties> for UserProfile {
    fn from(properties: UserProfileProperties) -> UserProfile {
        UserProfile { properties }
    }
}

/// The [`AWS::OpsWorks::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html) resource.
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(rename="Ec2VolumeId")]
    pub ec2_volume_id: String,
    #[serde(rename="MountPoint")]
    pub mount_point: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="StackId")]
    pub stack_id: String,
}

impl<'a> ::Resource<'a> for Volume {
    type Properties = VolumeProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Volume";
    fn properties(&self) -> &VolumeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VolumeProperties {
        &mut self.properties
    }
}

impl From<VolumeProperties> for Volume {
    fn from(properties: VolumeProperties) -> Volume {
        Volume { properties }
    }
}

pub mod app {
    #[derive(Serialize, Deserialize)]
    pub struct DataSource {
        #[serde(rename="Arn")]
        pub arn: String,
        #[serde(rename="DatabaseName")]
        pub database_name: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Secure")]
        pub secure: bool,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Source {
        #[serde(rename="Password")]
        pub password: String,
        #[serde(rename="Revision")]
        pub revision: String,
        #[serde(rename="SshKey")]
        pub ssh_key: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Url")]
        pub url: String,
        #[serde(rename="Username")]
        pub username: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SslConfiguration {
        #[serde(rename="Certificate")]
        pub certificate: String,
        #[serde(rename="Chain")]
        pub chain: String,
        #[serde(rename="PrivateKey")]
        pub private_key: String,
    }

}

pub mod instance {
    #[derive(Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        #[serde(rename="DeviceName")]
        pub device_name: String,
        #[serde(rename="Ebs")]
        pub ebs: EbsBlockDevice,
        #[serde(rename="NoDevice")]
        pub no_device: String,
        #[serde(rename="VirtualName")]
        pub virtual_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EbsBlockDevice {
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="SnapshotId")]
        pub snapshot_id: String,
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TimeBasedAutoScaling {
        #[serde(rename="Friday")]
        pub friday: ::std::collections::HashMap<String, String>,
        #[serde(rename="Monday")]
        pub monday: ::std::collections::HashMap<String, String>,
        #[serde(rename="Saturday")]
        pub saturday: ::std::collections::HashMap<String, String>,
        #[serde(rename="Sunday")]
        pub sunday: ::std::collections::HashMap<String, String>,
        #[serde(rename="Thursday")]
        pub thursday: ::std::collections::HashMap<String, String>,
        #[serde(rename="Tuesday")]
        pub tuesday: ::std::collections::HashMap<String, String>,
        #[serde(rename="Wednesday")]
        pub wednesday: ::std::collections::HashMap<String, String>,
    }

}

pub mod layer {
    #[derive(Serialize, Deserialize)]
    pub struct AutoScalingThresholds {
        #[serde(rename="CpuThreshold")]
        pub cpu_threshold: f64,
        #[serde(rename="IgnoreMetricsTime")]
        pub ignore_metrics_time: u32,
        #[serde(rename="InstanceCount")]
        pub instance_count: u32,
        #[serde(rename="LoadThreshold")]
        pub load_threshold: f64,
        #[serde(rename="MemoryThreshold")]
        pub memory_threshold: f64,
        #[serde(rename="ThresholdsWaitTime")]
        pub thresholds_wait_time: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LifecycleEventConfiguration {
        #[serde(rename="ShutdownEventConfiguration")]
        pub shutdown_event_configuration: ShutdownEventConfiguration,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoadBasedAutoScaling {
        #[serde(rename="DownScaling")]
        pub down_scaling: AutoScalingThresholds,
        #[serde(rename="Enable")]
        pub enable: bool,
        #[serde(rename="UpScaling")]
        pub up_scaling: AutoScalingThresholds,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Recipes {
        #[serde(rename="Configure")]
        pub configure: Vec<String>,
        #[serde(rename="Deploy")]
        pub deploy: Vec<String>,
        #[serde(rename="Setup")]
        pub setup: Vec<String>,
        #[serde(rename="Shutdown")]
        pub shutdown: Vec<String>,
        #[serde(rename="Undeploy")]
        pub undeploy: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ShutdownEventConfiguration {
        #[serde(rename="DelayUntilElbConnectionsDrained")]
        pub delay_until_elb_connections_drained: bool,
        #[serde(rename="ExecutionTimeout")]
        pub execution_timeout: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VolumeConfiguration {
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="MountPoint")]
        pub mount_point: String,
        #[serde(rename="NumberOfDisks")]
        pub number_of_disks: u32,
        #[serde(rename="RaidLevel")]
        pub raid_level: u32,
        #[serde(rename="Size")]
        pub size: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

}

pub mod stack {
    #[derive(Serialize, Deserialize)]
    pub struct ChefConfiguration {
        #[serde(rename="BerkshelfVersion")]
        pub berkshelf_version: String,
        #[serde(rename="ManageBerkshelf")]
        pub manage_berkshelf: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ElasticIp {
        #[serde(rename="Ip")]
        pub ip: String,
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RdsDbInstance {
        #[serde(rename="DbPassword")]
        pub db_password: String,
        #[serde(rename="DbUser")]
        pub db_user: String,
        #[serde(rename="RdsDbInstanceArn")]
        pub rds_db_instance_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Source {
        #[serde(rename="Password")]
        pub password: String,
        #[serde(rename="Revision")]
        pub revision: String,
        #[serde(rename="SshKey")]
        pub ssh_key: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Url")]
        pub url: String,
        #[serde(rename="Username")]
        pub username: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StackConfigurationManager {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Version")]
        pub version: String,
    }

}

