//! Types for the `OpsWorks` service.

/// The [`AWS::OpsWorks::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html) resource type.
#[derive(Debug)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppProperties {
    /// Property `AppSource`.
    #[serde(rename="AppSource")]
    pub app_source: self::app::Source,
    /// Property `Attributes`.
    #[serde(rename="Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// Property `DataSources`.
    #[serde(rename="DataSources")]
    pub data_sources: Vec<self::app::DataSource>,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Domains`.
    #[serde(rename="Domains")]
    pub domains: Vec<String>,
    /// Property `EnableSsl`.
    #[serde(rename="EnableSsl")]
    pub enable_ssl: bool,
    /// Property `Environment`.
    #[serde(rename="Environment")]
    pub environment: Vec<self::app::EnvironmentVariable>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Shortname`.
    #[serde(rename="Shortname")]
    pub shortname: String,
    /// Property `SslConfiguration`.
    #[serde(rename="SslConfiguration")]
    pub ssl_configuration: self::app::SslConfiguration,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
    /// Property `Type`.
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

impl ::private::Sealed for App {}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::OpsWorks::ElasticLoadBalancerAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html) resource type.
#[derive(Debug)]
pub struct ElasticLoadBalancerAttachment {
    properties: ElasticLoadBalancerAttachmentProperties
}

/// Properties for the `ElasticLoadBalancerAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ElasticLoadBalancerAttachmentProperties {
    /// Property `ElasticLoadBalancerName`.
    #[serde(rename="ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    /// Property `LayerId`.
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

impl ::private::Sealed for ElasticLoadBalancerAttachment {}

impl From<ElasticLoadBalancerAttachmentProperties> for ElasticLoadBalancerAttachment {
    fn from(properties: ElasticLoadBalancerAttachmentProperties) -> ElasticLoadBalancerAttachment {
        ElasticLoadBalancerAttachment { properties }
    }
}

/// The [`AWS::OpsWorks::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html) resource type.
#[derive(Debug)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceProperties {
    /// Property `AgentVersion`.
    #[serde(rename="AgentVersion")]
    pub agent_version: String,
    /// Property `AmiId`.
    #[serde(rename="AmiId")]
    pub ami_id: String,
    /// Property `Architecture`.
    #[serde(rename="Architecture")]
    pub architecture: String,
    /// Property `AutoScalingType`.
    #[serde(rename="AutoScalingType")]
    pub auto_scaling_type: String,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    /// Property `BlockDeviceMappings`.
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: Vec<self::instance::BlockDeviceMapping>,
    /// Property `EbsOptimized`.
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: bool,
    /// Property `ElasticIps`.
    #[serde(rename="ElasticIps")]
    pub elastic_ips: Vec<String>,
    /// Property `Hostname`.
    #[serde(rename="Hostname")]
    pub hostname: String,
    /// Property `InstallUpdatesOnBoot`.
    #[serde(rename="InstallUpdatesOnBoot")]
    pub install_updates_on_boot: bool,
    /// Property `InstanceType`.
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    /// Property `LayerIds`.
    #[serde(rename="LayerIds")]
    pub layer_ids: Vec<String>,
    /// Property `Os`.
    #[serde(rename="Os")]
    pub os: String,
    /// Property `RootDeviceType`.
    #[serde(rename="RootDeviceType")]
    pub root_device_type: String,
    /// Property `SshKeyName`.
    #[serde(rename="SshKeyName")]
    pub ssh_key_name: String,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
    /// Property `SubnetId`.
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
    /// Property `Tenancy`.
    #[serde(rename="Tenancy")]
    pub tenancy: String,
    /// Property `TimeBasedAutoScaling`.
    #[serde(rename="TimeBasedAutoScaling")]
    pub time_based_auto_scaling: self::instance::TimeBasedAutoScaling,
    /// Property `VirtualizationType`.
    #[serde(rename="VirtualizationType")]
    pub virtualization_type: String,
    /// Property `Volumes`.
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

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::OpsWorks::Layer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html) resource type.
#[derive(Debug)]
pub struct Layer {
    properties: LayerProperties
}

/// Properties for the `Layer` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerProperties {
    /// Property `Attributes`.
    #[serde(rename="Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// Property `AutoAssignElasticIps`.
    #[serde(rename="AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: bool,
    /// Property `AutoAssignPublicIps`.
    #[serde(rename="AutoAssignPublicIps")]
    pub auto_assign_public_ips: bool,
    /// Property `CustomInstanceProfileArn`.
    #[serde(rename="CustomInstanceProfileArn")]
    pub custom_instance_profile_arn: String,
    /// Property `CustomJson`.
    #[serde(rename="CustomJson")]
    pub custom_json: ::json::Value,
    /// Property `CustomRecipes`.
    #[serde(rename="CustomRecipes")]
    pub custom_recipes: self::layer::Recipes,
    /// Property `CustomSecurityGroupIds`.
    #[serde(rename="CustomSecurityGroupIds")]
    pub custom_security_group_ids: Vec<String>,
    /// Property `EnableAutoHealing`.
    #[serde(rename="EnableAutoHealing")]
    pub enable_auto_healing: bool,
    /// Property `InstallUpdatesOnBoot`.
    #[serde(rename="InstallUpdatesOnBoot")]
    pub install_updates_on_boot: bool,
    /// Property `LifecycleEventConfiguration`.
    #[serde(rename="LifecycleEventConfiguration")]
    pub lifecycle_event_configuration: self::layer::LifecycleEventConfiguration,
    /// Property `LoadBasedAutoScaling`.
    #[serde(rename="LoadBasedAutoScaling")]
    pub load_based_auto_scaling: self::layer::LoadBasedAutoScaling,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Packages`.
    #[serde(rename="Packages")]
    pub packages: Vec<String>,
    /// Property `Shortname`.
    #[serde(rename="Shortname")]
    pub shortname: String,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
    /// Property `UseEbsOptimizedInstances`.
    #[serde(rename="UseEbsOptimizedInstances")]
    pub use_ebs_optimized_instances: bool,
    /// Property `VolumeConfigurations`.
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

impl ::private::Sealed for Layer {}

impl From<LayerProperties> for Layer {
    fn from(properties: LayerProperties) -> Layer {
        Layer { properties }
    }
}

/// The [`AWS::OpsWorks::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html) resource type.
#[derive(Debug)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct StackProperties {
    /// Property `AgentVersion`.
    #[serde(rename="AgentVersion")]
    pub agent_version: String,
    /// Property `Attributes`.
    #[serde(rename="Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// Property `ChefConfiguration`.
    #[serde(rename="ChefConfiguration")]
    pub chef_configuration: self::stack::ChefConfiguration,
    /// Property `CloneAppIds`.
    #[serde(rename="CloneAppIds")]
    pub clone_app_ids: Vec<String>,
    /// Property `ClonePermissions`.
    #[serde(rename="ClonePermissions")]
    pub clone_permissions: bool,
    /// Property `ConfigurationManager`.
    #[serde(rename="ConfigurationManager")]
    pub configuration_manager: self::stack::StackConfigurationManager,
    /// Property `CustomCookbooksSource`.
    #[serde(rename="CustomCookbooksSource")]
    pub custom_cookbooks_source: self::stack::Source,
    /// Property `CustomJson`.
    #[serde(rename="CustomJson")]
    pub custom_json: ::json::Value,
    /// Property `DefaultAvailabilityZone`.
    #[serde(rename="DefaultAvailabilityZone")]
    pub default_availability_zone: String,
    /// Property `DefaultInstanceProfileArn`.
    #[serde(rename="DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,
    /// Property `DefaultOs`.
    #[serde(rename="DefaultOs")]
    pub default_os: String,
    /// Property `DefaultRootDeviceType`.
    #[serde(rename="DefaultRootDeviceType")]
    pub default_root_device_type: String,
    /// Property `DefaultSshKeyName`.
    #[serde(rename="DefaultSshKeyName")]
    pub default_ssh_key_name: String,
    /// Property `DefaultSubnetId`.
    #[serde(rename="DefaultSubnetId")]
    pub default_subnet_id: String,
    /// Property `EcsClusterArn`.
    #[serde(rename="EcsClusterArn")]
    pub ecs_cluster_arn: String,
    /// Property `ElasticIps`.
    #[serde(rename="ElasticIps")]
    pub elastic_ips: Vec<self::stack::ElasticIp>,
    /// Property `HostnameTheme`.
    #[serde(rename="HostnameTheme")]
    pub hostname_theme: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RdsDbInstances`.
    #[serde(rename="RdsDbInstances")]
    pub rds_db_instances: Vec<self::stack::RdsDbInstance>,
    /// Property `ServiceRoleArn`.
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    /// Property `SourceStackId`.
    #[serde(rename="SourceStackId")]
    pub source_stack_id: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `UseCustomCookbooks`.
    #[serde(rename="UseCustomCookbooks")]
    pub use_custom_cookbooks: bool,
    /// Property `UseOpsworksSecurityGroups`.
    #[serde(rename="UseOpsworksSecurityGroups")]
    pub use_opsworks_security_groups: bool,
    /// Property `VpcId`.
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

impl ::private::Sealed for Stack {}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::OpsWorks::UserProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html) resource type.
#[derive(Debug)]
pub struct UserProfile {
    properties: UserProfileProperties
}

/// Properties for the `UserProfile` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileProperties {
    /// Property `AllowSelfManagement`.
    #[serde(rename="AllowSelfManagement")]
    pub allow_self_management: bool,
    /// Property `IamUserArn`.
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
    /// Property `SshPublicKey`.
    #[serde(rename="SshPublicKey")]
    pub ssh_public_key: String,
    /// Property `SshUsername`.
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

impl ::private::Sealed for UserProfile {}

impl From<UserProfileProperties> for UserProfile {
    fn from(properties: UserProfileProperties) -> UserProfile {
        UserProfile { properties }
    }
}

/// The [`AWS::OpsWorks::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html) resource type.
#[derive(Debug)]
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeProperties {
    /// Property `Ec2VolumeId`.
    #[serde(rename="Ec2VolumeId")]
    pub ec2_volume_id: String,
    /// Property `MountPoint`.
    #[serde(rename="MountPoint")]
    pub mount_point: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `StackId`.
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

impl ::private::Sealed for Volume {}

impl From<VolumeProperties> for Volume {
    fn from(properties: VolumeProperties) -> Volume {
        Volume { properties }
    }
}

pub mod app {
    //! Property types for the `App` resource.

    /// The [`AWS::OpsWorks::App.DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-datasource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DataSource {
        /// Property `Arn`.
        #[serde(rename="Arn")]
        pub arn: String,
        /// Property `DatabaseName`.
        #[serde(rename="DatabaseName")]
        pub database_name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::OpsWorks::App.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Secure`.
        #[serde(rename="Secure")]
        pub secure: bool,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::OpsWorks::App.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Password`.
        #[serde(rename="Password")]
        pub password: String,
        /// Property `Revision`.
        #[serde(rename="Revision")]
        pub revision: String,
        /// Property `SshKey`.
        #[serde(rename="SshKey")]
        pub ssh_key: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Url`.
        #[serde(rename="Url")]
        pub url: String,
        /// Property `Username`.
        #[serde(rename="Username")]
        pub username: String,
    }

    /// The [`AWS::OpsWorks::App.SslConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SslConfiguration {
        /// Property `Certificate`.
        #[serde(rename="Certificate")]
        pub certificate: String,
        /// Property `Chain`.
        #[serde(rename="Chain")]
        pub chain: String,
        /// Property `PrivateKey`.
        #[serde(rename="PrivateKey")]
        pub private_key: String,
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::OpsWorks::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename="DeviceName")]
        pub device_name: String,
        /// Property `Ebs`.
        #[serde(rename="Ebs")]
        pub ebs: EbsBlockDevice,
        /// Property `NoDevice`.
        #[serde(rename="NoDevice")]
        pub no_device: String,
        /// Property `VirtualName`.
        #[serde(rename="VirtualName")]
        pub virtual_name: String,
    }

    /// The [`AWS::OpsWorks::Instance.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDevice {
        /// Property `DeleteOnTermination`.
        #[serde(rename="DeleteOnTermination")]
        pub delete_on_termination: bool,
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `SnapshotId`.
        #[serde(rename="SnapshotId")]
        pub snapshot_id: String,
        /// Property `VolumeSize`.
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    /// The [`AWS::OpsWorks::Instance.TimeBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TimeBasedAutoScaling {
        /// Property `Friday`.
        #[serde(rename="Friday")]
        pub friday: ::std::collections::HashMap<String, String>,
        /// Property `Monday`.
        #[serde(rename="Monday")]
        pub monday: ::std::collections::HashMap<String, String>,
        /// Property `Saturday`.
        #[serde(rename="Saturday")]
        pub saturday: ::std::collections::HashMap<String, String>,
        /// Property `Sunday`.
        #[serde(rename="Sunday")]
        pub sunday: ::std::collections::HashMap<String, String>,
        /// Property `Thursday`.
        #[serde(rename="Thursday")]
        pub thursday: ::std::collections::HashMap<String, String>,
        /// Property `Tuesday`.
        #[serde(rename="Tuesday")]
        pub tuesday: ::std::collections::HashMap<String, String>,
        /// Property `Wednesday`.
        #[serde(rename="Wednesday")]
        pub wednesday: ::std::collections::HashMap<String, String>,
    }
}

pub mod layer {
    //! Property types for the `Layer` resource.

    /// The [`AWS::OpsWorks::Layer.AutoScalingThresholds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingThresholds {
        /// Property `CpuThreshold`.
        #[serde(rename="CpuThreshold")]
        pub cpu_threshold: f64,
        /// Property `IgnoreMetricsTime`.
        #[serde(rename="IgnoreMetricsTime")]
        pub ignore_metrics_time: u32,
        /// Property `InstanceCount`.
        #[serde(rename="InstanceCount")]
        pub instance_count: u32,
        /// Property `LoadThreshold`.
        #[serde(rename="LoadThreshold")]
        pub load_threshold: f64,
        /// Property `MemoryThreshold`.
        #[serde(rename="MemoryThreshold")]
        pub memory_threshold: f64,
        /// Property `ThresholdsWaitTime`.
        #[serde(rename="ThresholdsWaitTime")]
        pub thresholds_wait_time: u32,
    }

    /// The [`AWS::OpsWorks::Layer.LifecycleEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecycleEventConfiguration {
        /// Property `ShutdownEventConfiguration`.
        #[serde(rename="ShutdownEventConfiguration")]
        pub shutdown_event_configuration: ShutdownEventConfiguration,
    }

    /// The [`AWS::OpsWorks::Layer.LoadBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBasedAutoScaling {
        /// Property `DownScaling`.
        #[serde(rename="DownScaling")]
        pub down_scaling: AutoScalingThresholds,
        /// Property `Enable`.
        #[serde(rename="Enable")]
        pub enable: bool,
        /// Property `UpScaling`.
        #[serde(rename="UpScaling")]
        pub up_scaling: AutoScalingThresholds,
    }

    /// The [`AWS::OpsWorks::Layer.Recipes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Recipes {
        /// Property `Configure`.
        #[serde(rename="Configure")]
        pub configure: Vec<String>,
        /// Property `Deploy`.
        #[serde(rename="Deploy")]
        pub deploy: Vec<String>,
        /// Property `Setup`.
        #[serde(rename="Setup")]
        pub setup: Vec<String>,
        /// Property `Shutdown`.
        #[serde(rename="Shutdown")]
        pub shutdown: Vec<String>,
        /// Property `Undeploy`.
        #[serde(rename="Undeploy")]
        pub undeploy: Vec<String>,
    }

    /// The [`AWS::OpsWorks::Layer.ShutdownEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShutdownEventConfiguration {
        /// Property `DelayUntilElbConnectionsDrained`.
        #[serde(rename="DelayUntilElbConnectionsDrained")]
        pub delay_until_elb_connections_drained: bool,
        /// Property `ExecutionTimeout`.
        #[serde(rename="ExecutionTimeout")]
        pub execution_timeout: u32,
    }

    /// The [`AWS::OpsWorks::Layer.VolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeConfiguration {
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `MountPoint`.
        #[serde(rename="MountPoint")]
        pub mount_point: String,
        /// Property `NumberOfDisks`.
        #[serde(rename="NumberOfDisks")]
        pub number_of_disks: u32,
        /// Property `RaidLevel`.
        #[serde(rename="RaidLevel")]
        pub raid_level: u32,
        /// Property `Size`.
        #[serde(rename="Size")]
        pub size: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }
}

pub mod stack {
    //! Property types for the `Stack` resource.

    /// The [`AWS::OpsWorks::Stack.ChefConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChefConfiguration {
        /// Property `BerkshelfVersion`.
        #[serde(rename="BerkshelfVersion")]
        pub berkshelf_version: String,
        /// Property `ManageBerkshelf`.
        #[serde(rename="ManageBerkshelf")]
        pub manage_berkshelf: bool,
    }

    /// The [`AWS::OpsWorks::Stack.ElasticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticIp {
        /// Property `Ip`.
        #[serde(rename="Ip")]
        pub ip: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::OpsWorks::Stack.RdsDbInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RdsDbInstance {
        /// Property `DbPassword`.
        #[serde(rename="DbPassword")]
        pub db_password: String,
        /// Property `DbUser`.
        #[serde(rename="DbUser")]
        pub db_user: String,
        /// Property `RdsDbInstanceArn`.
        #[serde(rename="RdsDbInstanceArn")]
        pub rds_db_instance_arn: String,
    }

    /// The [`AWS::OpsWorks::Stack.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Password`.
        #[serde(rename="Password")]
        pub password: String,
        /// Property `Revision`.
        #[serde(rename="Revision")]
        pub revision: String,
        /// Property `SshKey`.
        #[serde(rename="SshKey")]
        pub ssh_key: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Url`.
        #[serde(rename="Url")]
        pub url: String,
        /// Property `Username`.
        #[serde(rename="Username")]
        pub username: String,
    }

    /// The [`AWS::OpsWorks::Stack.StackConfigurationManager`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StackConfigurationManager {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Version`.
        #[serde(rename="Version")]
        pub version: String,
    }
}
